use std::pin::pin;
use std::sync::Arc;

use tokio::sync::Notify;
use tokio::sync::watch;
use tokio::time::Instant;

use margaret_jws_verification::verification_key_set::VerificationKeySet;

use crate::held_key_set::HeldKeySet;
use crate::key_set_holding::KeySetHolding;
use crate::key_set_polling::KeySetPolling;
use crate::key_set_refresh::KeySetRefresh;
use crate::key_set_snapshot::KeySetSnapshot;
use crate::key_set_state::KeySetState;
use crate::refresh_progress::RefreshProgress;

pub struct IssuerKeySet {
    completion: Notify,
    refresh: Notify,
    state: watch::Sender<KeySetState>,
}

impl IssuerKeySet {
    #[must_use]
    pub fn awaiting() -> Self {
        Self {
            completion: Notify::new(),
            refresh: Notify::new(),
            state: watch::Sender::new(KeySetState {
                completed_fetches: 0,
                holding: KeySetHolding::Awaiting,
                polling: KeySetPolling::Active,
                started_fetches: 0,
            }),
        }
    }

    pub fn fail_fetch(&self) {
        self.complete_fetch(|_holding| {});
    }

    pub fn hold(&self, key_set: Arc<VerificationKeySet>) {
        let held = KeySetHolding::Held(HeldKeySet {
            fetched_at: Instant::now(),
            key_set,
        });

        self.complete_fetch(|holding| *holding = held);
    }

    pub async fn refresh_requested(&self) {
        self.refresh.notified().await;
    }

    pub async fn refreshed_since(&self, snapshot: &KeySetSnapshot) -> KeySetRefresh {
        let mut completion = pin!(self.completion.notified());

        self.refresh.notify_one();

        loop {
            completion.as_mut().enable();

            if let RefreshProgress::Finished(refresh) = self.refresh_progress(snapshot) {
                return refresh;
            }

            completion.as_mut().await;
            completion.set(self.completion.notified());
        }
    }

    #[must_use]
    pub fn snapshot(&self) -> KeySetSnapshot {
        let state = self.state.borrow();

        KeySetSnapshot {
            holding: state.holding.clone(),
            started_fetches: state.started_fetches,
        }
    }

    pub fn start_fetch(&self) {
        self.state.send_modify(|state| state.started_fetches += 1);
    }

    pub fn stop_polling(&self) {
        self.state
            .send_modify(|state| state.polling = KeySetPolling::Stopped);
        self.completion.notify_waiters();
    }

    fn complete_fetch(&self, update: impl FnOnce(&mut KeySetHolding)) {
        self.state.send_modify(|state| {
            update(&mut state.holding);
            state.completed_fetches = state.started_fetches;
        });
        self.completion.notify_waiters();
    }

    fn refresh_progress(&self, snapshot: &KeySetSnapshot) -> RefreshProgress {
        let state = self.state.borrow();

        if state.completed_fetches > snapshot.started_fetches {
            RefreshProgress::Finished(KeySetRefresh::Refreshed(state.holding.clone()))
        } else if state.polling == KeySetPolling::Stopped {
            RefreshProgress::Finished(KeySetRefresh::PollingStopped)
        } else {
            RefreshProgress::Pending
        }
    }
}

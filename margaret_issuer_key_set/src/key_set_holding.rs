use tokio::time::Instant;

use crate::held_key_set::HeldKeySet;
use crate::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use crate::key_set_poll_interval_after_ready::KEY_SET_POLL_INTERVAL_AFTER_READY;

#[derive(Clone)]
pub enum KeySetHolding {
    Awaiting,
    Held(HeldKeySet),
}

impl KeySetHolding {
    #[must_use]
    pub fn next_fetch_due(&self, fetch_started_at: Instant) -> Instant {
        match self {
            Self::Awaiting => fetch_started_at + ISSUER_FETCH_SPACING,
            Self::Held(_) => fetch_started_at + KEY_SET_POLL_INTERVAL_AFTER_READY,
        }
    }
}

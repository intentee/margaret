use std::future::Future;

use tokio::time::MissedTickBehavior;
use tokio::time::interval;

use crate::constants::READINESS_POLL_INTERVAL;

pub async fn poll_until<TProbe: Future<Output = bool>>(probe: impl Fn() -> TProbe) {
    let mut polls = interval(READINESS_POLL_INTERVAL);

    polls.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while !probe().await {
        polls.tick().await;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use super::poll_until;

    #[tokio::test]
    async fn probes_again_after_a_failed_probe_until_one_passes() {
        let probes = AtomicUsize::new(0);
        let counted = &probes;

        poll_until(move || async move { counted.fetch_add(1, Ordering::SeqCst) == 1 }).await;

        assert_eq!(probes.load(Ordering::SeqCst), 2);
    }
}

use std::future::Future;

use tokio::time::MissedTickBehavior;
use tokio::time::interval;

use margaret_local_server_tests::readiness_poll_interval::READINESS_POLL_INTERVAL;

use crate::cluster_instance::ClusterInstance;

/// # Panics
///
/// Panics when the instance exits before the probe passes.
pub async fn await_probe<TProbe: Future<Output = bool>>(
    instance: &mut ClusterInstance,
    probe: impl Fn() -> TProbe,
) {
    let mut polls = interval(READINESS_POLL_INTERVAL);

    polls.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while !probe().await {
        assert!(
            !instance.has_exited(),
            "the instance exits before it becomes ready"
        );
        polls.tick().await;
    }
}

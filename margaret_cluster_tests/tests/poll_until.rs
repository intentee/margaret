use std::future::Future;

use tokio::time::MissedTickBehavior;
use tokio::time::interval;

use margaret_cluster_tests::readiness_poll_interval::READINESS_POLL_INTERVAL;

pub async fn poll_until<TProbe: Future<Output = bool>>(probe: impl Fn() -> TProbe) {
    let mut polls = interval(READINESS_POLL_INTERVAL);

    polls.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while !probe().await {
        polls.tick().await;
    }
}

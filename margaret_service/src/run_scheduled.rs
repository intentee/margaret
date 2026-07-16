use std::future::Future;
use std::time::Duration;

use tokio::time::MissedTickBehavior;
use tokio_util::sync::CancellationToken;

pub async fn run_scheduled<Runner, RunnerFuture, RunnerError>(
    period: Duration,
    missed_tick_behavior: MissedTickBehavior,
    cancellation_token: CancellationToken,
    mut runner: Runner,
) -> Result<(), RunnerError>
where
    Runner: FnMut() -> RunnerFuture,
    RunnerFuture: Future<Output = Result<(), RunnerError>>,
{
    let mut interval = tokio::time::interval(period);

    interval.set_missed_tick_behavior(missed_tick_behavior);

    loop {
        tokio::select! {
            () = cancellation_token.cancelled() => return Ok(()),
            _ = interval.tick() => runner().await?,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::time::Duration;

    use tokio::time::MissedTickBehavior;
    use tokio_util::sync::CancellationToken;

    use super::run_scheduled;

    #[tokio::test]
    async fn ticks_the_runner_then_stops_on_cancellation() {
        let cancellation_token = CancellationToken::new();
        let runner_token = cancellation_token.clone();
        let ticks = Arc::new(AtomicUsize::new(0));
        let runner_ticks = Arc::clone(&ticks);

        let outcome: Result<(), &str> = run_scheduled(
            Duration::from_millis(1),
            MissedTickBehavior::Delay,
            cancellation_token,
            move || {
                let runner_token = runner_token.clone();
                let runner_ticks = Arc::clone(&runner_ticks);

                async move {
                    runner_ticks.fetch_add(1, Ordering::SeqCst);
                    runner_token.cancel();

                    Ok(())
                }
            },
        )
        .await;

        assert_eq!(outcome, Ok(()));
        assert_eq!(ticks.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn propagates_a_runner_error() {
        let outcome: Result<(), &str> = run_scheduled(
            Duration::from_millis(1),
            MissedTickBehavior::Delay,
            CancellationToken::new(),
            || async { Err("scheduled runner failed") },
        )
        .await;

        assert_eq!(outcome, Err("scheduled runner failed"));
    }
}

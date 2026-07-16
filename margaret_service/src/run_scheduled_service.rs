use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tokio::time::MissedTickBehavior;
use tokio_util::sync::CancellationToken;

use crate::run_scheduled::run_scheduled;
use crate::tick_runner::TickRunner;

pub async fn run_scheduled_service<Runner: TickRunner>(
    period: Duration,
    missed_tick_behavior: MissedTickBehavior,
    cancellation_token: CancellationToken,
    runner: Arc<Runner>,
) -> Result<()> {
    run_scheduled(
        period,
        missed_tick_behavior,
        cancellation_token.clone(),
        move || {
            let runner = Arc::clone(&runner);
            let cancellation_token = cancellation_token.clone();

            async move { runner.tick(cancellation_token).await }
        },
    )
    .await
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use anyhow::Result;
    use anyhow::anyhow;
    use async_trait::async_trait;
    use tokio::time::MissedTickBehavior;
    use tokio_util::sync::CancellationToken;

    use super::run_scheduled_service;
    use crate::tick_runner::TickRunner;

    struct CancelsOnTick;

    #[async_trait]
    impl TickRunner for CancelsOnTick {
        async fn tick(&self, cancellation_token: CancellationToken) -> Result<()> {
            cancellation_token.cancel();

            Ok(())
        }
    }

    struct FailsOnTick;

    #[async_trait]
    impl TickRunner for FailsOnTick {
        async fn tick(&self, _cancellation_token: CancellationToken) -> Result<()> {
            Err(anyhow!("the tick failed"))
        }
    }

    #[tokio::test]
    async fn runs_the_service_until_a_tick_requests_cancellation() {
        run_scheduled_service(
            Duration::from_millis(1),
            MissedTickBehavior::Delay,
            CancellationToken::new(),
            Arc::new(CancelsOnTick),
        )
        .await
        .expect("the service stops cleanly on cancellation");
    }

    #[tokio::test]
    async fn propagates_a_tick_failure() {
        let error = run_scheduled_service(
            Duration::from_millis(1),
            MissedTickBehavior::Delay,
            CancellationToken::new(),
            Arc::new(FailsOnTick),
        )
        .await
        .expect_err("a failing tick stops the service");

        assert_eq!(error.to_string(), "the tick failed");
    }
}

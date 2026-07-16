use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tokio::time::MissedTickBehavior;
use tokio_util::sync::CancellationToken;

use crate::run_scheduled::run_scheduled;
use crate::scheduled_argument_runner::ScheduledArgumentRunner;

pub async fn run_scheduled_service_with_argument<Runner, Argument>(
    period: Duration,
    missed_tick_behavior: MissedTickBehavior,
    cancellation_token: CancellationToken,
    runner: Arc<Runner>,
    argument: Argument,
) -> Result<()>
where
    Runner: ScheduledArgumentRunner<Argument>,
    Argument: Clone + Send + 'static,
{
    run_scheduled(
        period,
        missed_tick_behavior,
        cancellation_token.clone(),
        move || {
            let runner = Arc::clone(&runner);
            let argument = argument.clone();

            async move { runner.run_scheduled_tick(argument).await }
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

    use super::run_scheduled_service_with_argument;
    use crate::scheduled_argument_runner::ScheduledArgumentRunner;

    struct CancelsOnTick {
        cancellation_token: CancellationToken,
    }

    #[async_trait]
    impl ScheduledArgumentRunner<String> for CancelsOnTick {
        async fn run_scheduled_tick(&self, argument: String) -> Result<()> {
            assert_eq!(argument, "argument");
            self.cancellation_token.cancel();

            Ok(())
        }
    }

    struct FailsOnTick;

    #[async_trait]
    impl ScheduledArgumentRunner<String> for FailsOnTick {
        async fn run_scheduled_tick(&self, _argument: String) -> Result<()> {
            Err(anyhow!("the tick failed"))
        }
    }

    #[tokio::test]
    async fn passes_the_argument_until_a_tick_requests_cancellation() {
        let cancellation_token = CancellationToken::new();

        run_scheduled_service_with_argument(
            Duration::from_millis(1),
            MissedTickBehavior::Delay,
            cancellation_token.clone(),
            Arc::new(CancelsOnTick { cancellation_token }),
            "argument".to_string(),
        )
        .await
        .expect("the service stops cleanly on cancellation");
    }

    #[tokio::test]
    async fn propagates_a_tick_failure() {
        let error = run_scheduled_service_with_argument(
            Duration::from_millis(1),
            MissedTickBehavior::Delay,
            CancellationToken::new(),
            Arc::new(FailsOnTick),
            "argument".to_string(),
        )
        .await
        .expect_err("a failing tick stops the service");

        assert_eq!(error.to_string(), "the tick failed");
    }
}

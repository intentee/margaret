use trzcina::Service;
use trzcina::ServiceBundle;

use margaret_console::command_outcome::CommandOutcome;
use margaret_console::report_failure::report_failure;

pub async fn bundle_services<TServiceBundle: ServiceBundle>(
    bundle: TServiceBundle,
) -> Result<Vec<Box<dyn Service>>, CommandOutcome> {
    bundle.services().await.map_err(report_failure)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use anyhow::bail;
    use async_trait::async_trait;
    use tokio_util::sync::CancellationToken;
    use trzcina::Service;
    use trzcina::ServiceBundle;

    use margaret_console::command_outcome::CommandOutcome;

    use super::bundle_services;

    struct Completes;

    #[async_trait]
    impl Service for Completes {
        async fn run(self: Box<Self>, _cancellation_token: CancellationToken) -> Result<()> {
            Ok(())
        }
    }

    struct FailingBundle;

    #[async_trait]
    impl ServiceBundle for FailingBundle {
        async fn services(self) -> Result<Vec<Box<dyn Service>>> {
            bail!("the bundle could not provide its services")
        }
    }

    struct ProvidingBundle;

    #[async_trait]
    impl ServiceBundle for ProvidingBundle {
        async fn services(self) -> Result<Vec<Box<dyn Service>>> {
            Ok(vec![Box::new(Completes)])
        }
    }

    #[tokio::test]
    async fn resolves_the_services_a_bundle_provides() {
        let mut services = bundle_services(ProvidingBundle)
            .await
            .expect("the bundle provides its services");
        let service = services.pop().expect("the bundle provides one service");

        assert!(services.is_empty());

        service
            .run(CancellationToken::new())
            .await
            .expect("the resolved service runs");
    }

    #[tokio::test]
    async fn reports_a_failure_when_the_bundle_cannot_provide_its_services() {
        let outcome = bundle_services(FailingBundle).await.err();

        assert_eq!(outcome, Some(CommandOutcome::Failed));
    }
}

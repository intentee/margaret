use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_console::command_outcome::CommandOutcome;
use margaret_service::Service;
use margaret_service::ServiceManager;
use margaret_service::ServiceShutdownOptions;
use margaret_service::run::run;

struct Completes;

#[async_trait]
impl Service for Completes {
    async fn run(self: Box<Self>, _cancellation_token: CancellationToken) -> Result<()> {
        Ok(())
    }
}

struct Fails;

#[async_trait]
impl Service for Fails {
    async fn run(self: Box<Self>, _cancellation_token: CancellationToken) -> Result<()> {
        Err(anyhow!("the service failed"))
    }
}

#[tokio::test]
async fn maps_clean_completion_to_succeeded() {
    let mut manager = ServiceManager::default();

    manager.register_service(Completes);

    let outcome = run(
        manager,
        CancellationToken::new(),
        ServiceShutdownOptions::default(),
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

#[tokio::test]
async fn maps_a_failing_service_to_failed() {
    let mut manager = ServiceManager::default();

    manager.register_service(Fails);

    let outcome = run(
        manager,
        CancellationToken::new(),
        ServiceShutdownOptions::default(),
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Failed);
}

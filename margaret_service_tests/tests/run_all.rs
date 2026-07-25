use std::time::Duration;

use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use trzcina::Service;
use trzcina::ServiceManager;
use trzcina::ServiceShutdownOptions;

use margaret_console::command_outcome::CommandOutcome;
use margaret_service::run_all::run_all;

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

struct IgnoresCancellation;

#[async_trait]
impl Service for IgnoresCancellation {
    async fn run(self: Box<Self>, _cancellation_token: CancellationToken) -> Result<()> {
        std::future::pending::<()>().await;

        Ok(())
    }
}

#[tokio::test]
async fn drives_multiple_managers_and_fails_when_any_manager_fails() {
    let cancellation_token = CancellationToken::new();

    let mut completing_manager = ServiceManager::default();
    completing_manager.register_service(Completes);

    let mut failing_manager = ServiceManager::default();
    failing_manager.register_service(Fails);

    let outcome = run_all(
        vec![
            completing_manager.start(cancellation_token.clone()),
            failing_manager.start(cancellation_token.clone()),
        ],
        ServiceShutdownOptions::default(),
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Failed);
}

#[tokio::test]
async fn drives_multiple_managers_and_succeeds_when_every_manager_completes() {
    let cancellation_token = CancellationToken::new();

    let mut first_manager = ServiceManager::default();
    first_manager.register_service(Completes);

    let mut second_manager = ServiceManager::default();
    second_manager.register_service(Completes);

    let outcome = run_all(
        vec![
            first_manager.start(cancellation_token.clone()),
            second_manager.start(cancellation_token.clone()),
        ],
        ServiceShutdownOptions::default(),
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

#[tokio::test(start_paused = true)]
async fn enforces_shutdown_deadlines_across_managers_within_a_single_window() {
    let deadline = Duration::from_secs(5);
    let cancellation_token = CancellationToken::new();

    let mut first_manager = ServiceManager::default();
    first_manager.register_service(IgnoresCancellation);

    let mut second_manager = ServiceManager::default();
    second_manager.register_service(IgnoresCancellation);

    let collections = vec![
        first_manager.start(cancellation_token.clone()),
        second_manager.start(cancellation_token.clone()),
    ];

    cancellation_token.cancel();

    let started_at = Instant::now();
    run_all(
        collections,
        ServiceShutdownOptions {
            cooperative_deadline: deadline,
            abort_deadline: deadline,
        },
    )
    .await;
    let elapsed = started_at.elapsed();

    assert!(elapsed >= deadline);
    assert!(elapsed < deadline * 2);
}

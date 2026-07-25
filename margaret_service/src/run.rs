use tokio_util::sync::CancellationToken;
use trzcina::ServiceManager;
use trzcina::ServiceShutdownOptions;

use margaret_console::command_outcome::CommandOutcome;

use crate::run_all::run_all;

pub async fn run(
    manager: ServiceManager,
    cancellation_token: CancellationToken,
    options: ServiceShutdownOptions,
) -> CommandOutcome {
    run_all(vec![manager.start(cancellation_token)], options).await
}

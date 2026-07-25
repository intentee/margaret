use tokio_util::sync::CancellationToken;
use trzcina::ServiceManager;
use trzcina::ServiceShutdownOptions;

use margaret_console::command_outcome::CommandOutcome;

pub async fn run(
    manager: ServiceManager,
    cancellation_token: CancellationToken,
    options: ServiceShutdownOptions,
) -> CommandOutcome {
    match manager
        .start(cancellation_token)
        .run_to_completion(options)
        .await
        .into_result()
    {
        Ok(()) => CommandOutcome::Succeeded,
        Err(error) => {
            eprintln!("{error}");

            CommandOutcome::Failed
        }
    }
}

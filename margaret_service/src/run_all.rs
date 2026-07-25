use futures_util::future::join_all;
use trzcina::RunningServiceCollection;
use trzcina::ServiceShutdownOptions;

use margaret_console::command_outcome::CommandOutcome;

pub async fn run_all(
    collections: Vec<RunningServiceCollection>,
    options: ServiceShutdownOptions,
) -> CommandOutcome {
    let shutdowns = join_all(
        collections
            .into_iter()
            .map(|collection| collection.run_to_completion(options.clone())),
    )
    .await;

    let mut outcome = CommandOutcome::Succeeded;

    for shutdown in shutdowns {
        if let Err(error) = shutdown.into_result() {
            eprintln!("{error}");

            outcome = CommandOutcome::Failed;
        }
    }

    outcome
}

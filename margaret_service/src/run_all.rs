use trzcina::RunningServiceCollection;
use trzcina::ServiceShutdownOptions;

use margaret_console::command_outcome::CommandOutcome;

pub async fn run_all(
    collections: Vec<RunningServiceCollection>,
    options: ServiceShutdownOptions,
) -> CommandOutcome {
    let mut outcome = CommandOutcome::Succeeded;

    for collection in collections {
        if let Err(error) = collection
            .run_to_completion(options.clone())
            .await
            .into_result()
        {
            eprintln!("{error}");

            outcome = CommandOutcome::Failed;
        }
    }

    outcome
}

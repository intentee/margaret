use margaret_console::command_outcome::CommandOutcome;
use margaret_identity::margaret::container::build::build;
use margaret_identity::margaret::run::run;

#[tokio::test]
async fn run_prints_help_without_a_subcommand() {
    let container = build();

    let outcome = run(&container, ["identity"]).await;

    assert_eq!(outcome, CommandOutcome::Failed);
}

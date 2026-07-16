use margaret_console::command_outcome::CommandOutcome;
use margaret_identity::margaret::container::build::build;
use margaret_identity::margaret::run::run;

#[tokio::test]
async fn run_reports_a_usage_error_for_an_unknown_flag() {
    let container = build();

    let outcome = run(&container, ["identity", "--nonexistent-flag"]).await;

    assert_eq!(outcome, CommandOutcome::Failed);
}

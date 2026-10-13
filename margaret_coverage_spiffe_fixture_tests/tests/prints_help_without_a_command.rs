use margaret_coverage_spiffe_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_spiffe_fixture::margaret::run::run;

#[tokio::test]
async fn prints_help_without_a_command() {
    assert_eq!(
        run(["margaret_coverage_spiffe_fixture"]).await,
        CommandOutcome::Failed
    );
}

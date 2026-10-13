use margaret_coverage_spiffe_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_spiffe_fixture::margaret::run::run;

#[tokio::test]
async fn refuses_to_serve_without_a_trust_domain() {
    assert_eq!(
        run(["margaret_coverage_spiffe_fixture", "serve"]).await,
        CommandOutcome::Failed
    );
}

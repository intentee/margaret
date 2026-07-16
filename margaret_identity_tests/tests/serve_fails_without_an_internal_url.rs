use margaret_console::command_outcome::CommandOutcome;
use margaret_identity::margaret::container::build::build;
use margaret_identity::margaret::serve::serve;
use margaret_identity_tests::serve_matches::serve_matches;
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn serve_fails_without_an_internal_url() {
    let container = build();
    let matches = serve_matches(&[
        "--spiffe-trust-domain",
        "example.org",
        "--spire-agent-addr",
        "unix:///run/spire-agent.sock",
    ]);

    let outcome = serve(&container, &matches, CancellationToken::new()).await;

    assert_eq!(outcome, CommandOutcome::Failed);
}

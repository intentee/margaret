use margaret_console::command_outcome::CommandOutcome;
use margaret_identity::margaret::container::build::build;
use margaret_identity::margaret::serve::serve;
use margaret_identity_tests::serve_matches::serve_matches;
use tokio_util::sync::CancellationToken;

#[tokio::test(flavor = "multi_thread")]
async fn serve_runs_to_completion_when_cancelled() {
    let container = build();
    let matches = serve_matches(&[
        "--spiffe-trust-domain",
        "example.org",
        "--spire-agent-addr",
        "unix:///run/spire-agent.sock",
        "--internal-url",
        "https://internal.example.org",
        "--public-url",
        "https://public.example.org",
        "--internal-addr",
        "127.0.0.1:0",
        "--public-addr",
        "127.0.0.1:0",
        "--public-transport",
        "plain",
        "--jwks-secret-path",
        "jwks.json",
    ]);
    let cancellation_token = CancellationToken::new();
    cancellation_token.cancel();

    let outcome = serve(&container, &matches, cancellation_token).await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

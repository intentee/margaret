use margaret_console::command_outcome::CommandOutcome;
use margaret_identity::margaret::container::build::build;
use margaret_identity::margaret::run::run;

#[tokio::test(flavor = "multi_thread")]
async fn run_dispatches_the_serve_command() {
    let container = build();

    let outcome = run(
        &container,
        [
            "identity",
            "serve",
            "--internal-addr",
            "not-an-address",
            "--internal-url",
            "https://internal.example.org",
            "--internal-transport",
            "spiffe_mtls",
            "--public-addr",
            "127.0.0.1:0",
            "--public-url",
            "https://public.example.org",
            "--public-transport",
            "plain",
            "--spiffe-trust-domain",
            "example.org",
            "--spire-agent-addr",
            "unix:///run/spire-agent.sock",
            "--jwks-secret-path",
            "jwks.json",
        ],
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Failed);
}

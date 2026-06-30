use clap::Arg;
use clap::Command;
use tokio_util::sync::CancellationToken;

use margaret_console::command_outcome::CommandOutcome;
use margaret_example::margaret::container::Container;
use margaret_example::margaret::services::serve;

#[tokio::test]
async fn runs_services_until_cancelled() {
    let container = Container::build();
    let metrics = container.metrics_metrics().await;
    let matches = Command::new("test")
        .arg(Arg::new("public-addr").long("public-addr").required(true))
        .arg(
            Arg::new("internal-addr")
                .long("internal-addr")
                .required(true),
        )
        .try_get_matches_from([
            "test",
            "--public-addr",
            "127.0.0.1:0",
            "--internal-addr",
            "127.0.0.1:0",
        ])
        .expect("the serve arguments parse");

    let cancellation_token = CancellationToken::new();
    let cancel_token = cancellation_token.clone();
    let cancel_metrics = metrics.clone();
    let canceller = tokio::spawn(async move {
        cancel_metrics.wait_for_sweep().await;
        cancel_token.cancel();
    });

    let outcome = serve(&container, &matches, cancellation_token).await;

    canceller.await.expect("the canceller task completes");

    assert_eq!(outcome, CommandOutcome::Succeeded);
    assert!(metrics.sweeps() >= 1);
}

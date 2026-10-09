use std::process;

use margaret_process_tests::process_signal::ProcessSignal;
use margaret_process_tests::signal_process::signal_process;
use margaret_service::install::install;

#[tokio::test]
async fn receives_sigint() {
    let signals = install().expect("the shutdown signal handlers install");

    assert!(signal_process(process::id(), ProcessSignal::Interrupt).success());

    signals.wait().await.expect("SIGINT is received");
}

#[tokio::test]
async fn receives_sigterm() {
    let signals = install().expect("the shutdown signal handlers install");

    assert!(signal_process(process::id(), ProcessSignal::Terminate).success());

    signals.wait().await.expect("SIGTERM is received");
}

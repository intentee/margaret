use std::process;

use margaret_process_tests::process_signal::ProcessSignal;
use margaret_process_tests::signal_process::signal_process;
use margaret_service::install::install;

#[tokio::test]
async fn wakes_the_shutdown_wait_on_sigint_so_ctrl_c_stops_the_service_gracefully() {
    let signals = install().expect("the shutdown signal handlers install");

    assert!(signal_process(process::id(), ProcessSignal::Interrupt).success());

    signals.wait().await.expect("SIGINT is received");
}

#[tokio::test]
async fn wakes_the_shutdown_wait_on_sigterm_so_a_supervisor_can_stop_the_service_gracefully() {
    let signals = install().expect("the shutdown signal handlers install");

    assert!(signal_process(process::id(), ProcessSignal::Terminate).success());

    signals.wait().await.expect("SIGTERM is received");
}

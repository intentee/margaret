use std::process;

use tokio::time::interval;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture::margaret::framework::service::install::install;
use margaret_coverage_fixture::margaret::run::run;
use margaret_local_server_tests::constants::READINESS_POLL_INTERVAL;
use margaret_process_tests::process_signal::ProcessSignal;
use margaret_process_tests::signal_process::signal_process;

#[tokio::test]
async fn holds_the_library_open_until_it_is_stopped() {
    let _signals_sent_before_the_hold_listens = install().expect("the test absorbs early stops");
    let holding = run(["margaret_coverage_fixture", "hold"]);
    let mut stops = interval(READINESS_POLL_INTERVAL);

    tokio::pin!(holding);

    let outcome = loop {
        tokio::select! {
            outcome = &mut holding => break outcome,
            _ = stops.tick() => {
                assert!(signal_process(process::id(), ProcessSignal::Terminate).success());
            }
        }
    };

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

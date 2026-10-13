use std::ffi::OsStr;

use tempfile::tempdir;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture::margaret::run::run;

#[tokio::test]
async fn refuses_a_greeting_without_a_reader_name() {
    let directory = tempdir().expect("the greeting directory is created");
    let greeting = directory.path().join("greeting");

    assert_eq!(
        run([
            OsStr::new("margaret_coverage_fixture"),
            OsStr::new("greet"),
            OsStr::new("--to"),
            greeting.as_os_str(),
        ])
        .await,
        CommandOutcome::Failed
    );
    assert!(!greeting.exists());
}

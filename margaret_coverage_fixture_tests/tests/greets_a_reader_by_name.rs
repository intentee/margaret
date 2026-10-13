use std::ffi::OsStr;
use std::fs;

use tempfile::tempdir;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture::margaret::run::run;

#[tokio::test]
async fn greets_a_reader_by_name() {
    let directory = tempdir().expect("the greeting directory is created");
    let greeting = directory.path().join("greeting");

    assert_eq!(
        run([
            OsStr::new("margaret_coverage_fixture"),
            OsStr::new("greet"),
            OsStr::new("Ada"),
            OsStr::new("--to"),
            greeting.as_os_str(),
        ])
        .await,
        CommandOutcome::Succeeded
    );
    assert_eq!(
        fs::read_to_string(&greeting).expect("the greeting is written"),
        "Ada"
    );
}

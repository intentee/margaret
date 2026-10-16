use std::process::Command;

use tempfile::tempdir;

use margaret_process_tests::broken_pipe::broken_pipe;
use margaret_process_tests::constants::FAILURE_EXIT_CODE;

use crate::fixture_binary::fixture_binary;

#[test]
fn fails_when_a_command_failure_cannot_be_reported() {
    let directory = tempdir().expect("the inventory directory is created");

    assert_eq!(
        Command::new(fixture_binary())
            .arg("count-books")
            .arg("--inventory")
            .arg(directory.path().join("missing-inventory"))
            .arg("--to")
            .arg(directory.path().join("count"))
            .stderr(broken_pipe())
            .status()
            .expect("the fixture binary runs")
            .code(),
        Some(FAILURE_EXIT_CODE)
    );
}

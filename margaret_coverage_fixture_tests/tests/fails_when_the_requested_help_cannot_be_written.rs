use std::process::Command;

use margaret_process_tests::broken_pipe::broken_pipe;
use margaret_process_tests::constants::FAILURE_EXIT_CODE;

use crate::fixture_binary::fixture_binary;

#[test]
fn fails_when_the_requested_help_cannot_be_written() {
    assert_eq!(
        Command::new(fixture_binary())
            .arg("--help")
            .stdout(broken_pipe())
            .status()
            .expect("the fixture binary runs")
            .code(),
        Some(FAILURE_EXIT_CODE)
    );
}

use std::process::Command;

use margaret_process_tests::broken_pipe::broken_pipe;
use margaret_process_tests::constants::FAILURE_EXIT_CODE;

use crate::cluster_binary::cluster_binary;

#[test]
fn fails_when_the_schema_cannot_be_written() {
    assert_eq!(
        Command::new(cluster_binary())
            .arg("schema")
            .stdout(broken_pipe())
            .status()
            .expect("the cluster binary runs")
            .code(),
        Some(FAILURE_EXIT_CODE)
    );
}

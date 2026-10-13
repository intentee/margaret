use std::process::Command;

use crate::cluster_binary::cluster_binary;

#[test]
fn prints_help_without_a_command() {
    assert!(
        !Command::new(cluster_binary())
            .status()
            .expect("the cluster binary runs")
            .success()
    );
}

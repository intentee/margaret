use margaret_cluster_tests::unreachable_database_command::unreachable_database_command;

use crate::cluster_binary::cluster_binary;

#[test]
fn refuses_to_write_a_note_without_a_reachable_database() {
    assert!(
        !unreachable_database_command(cluster_binary())
            .args(["write-note", "first draft"])
            .status()
            .expect("the cluster binary runs")
            .success()
    );
}

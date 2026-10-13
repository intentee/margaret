use margaret_cluster_tests::unreachable_database_command::unreachable_database_command;

use crate::cluster_binary::cluster_binary;

#[test]
fn refuses_to_seed_without_a_reachable_database() {
    assert!(
        !unreachable_database_command(cluster_binary())
            .arg("seed")
            .status()
            .expect("the cluster binary runs")
            .success()
    );
}

use std::time::Duration;

use margaret_spiffe_svid_manager_tests::spire_server_exits_immediately::spire_server_exits_immediately;
use margaret_spiffe_svid_manager_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_manager_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn errors_when_server_exits_before_creating_socket() {
    let result = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams {
            server_binary_path: spire_server_exits_immediately(),
            server_socket_readiness_timeout: Duration::from_millis(100),
            ..Default::default()
        },
        20010,
    )
    .await;

    assert!(result.is_err());
}

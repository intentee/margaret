use std::path::PathBuf;

use margaret_spiffe_svid_manager_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_manager_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn errors_when_server_binary_does_not_exist() {
    let result = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams {
            server_binary_path: PathBuf::from("/nonexistent/spire-server-coverage-test"),
            ..Default::default()
        },
        20008,
    )
    .await;

    assert!(result.is_err());
}

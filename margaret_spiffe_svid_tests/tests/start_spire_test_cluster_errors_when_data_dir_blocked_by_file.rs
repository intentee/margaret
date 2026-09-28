use tokio::fs;

use margaret_spiffe_svid_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn errors_when_server_data_dir_path_is_blocked_by_a_file() {
    let data_dir = tempfile::tempdir().unwrap();

    fs::write(data_dir.path().join("server-data"), b"blocker")
        .await
        .unwrap();

    let result = SpireTestCluster::start(data_dir, SpireTestClusterParams::default(), 20005).await;

    assert!(result.is_err());
}

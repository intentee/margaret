use margaret_spiffe_svid_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn errors_when_server_conf_path_is_a_directory() {
    let data_dir = tempfile::tempdir().unwrap();

    tokio::fs::create_dir_all(data_dir.path().join("server.conf"))
        .await
        .unwrap();

    let result = SpireTestCluster::start(data_dir, SpireTestClusterParams::default(), 20009).await;

    assert!(result.is_err());
}

use margaret_spiffe_svid_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn errors_when_trust_bundle_path_is_blocked_by_a_directory() {
    let data_dir = tempfile::tempdir().unwrap();

    tokio::fs::create_dir(data_dir.path().join("trust-bundle.pem"))
        .await
        .unwrap();

    let result = SpireTestCluster::start(data_dir, SpireTestClusterParams::default(), 20011).await;

    assert!(result.is_err());
}

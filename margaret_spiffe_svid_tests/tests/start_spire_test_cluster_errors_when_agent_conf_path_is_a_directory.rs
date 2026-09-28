use tokio::fs;

use margaret_spiffe_svid_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn errors_when_agent_conf_path_is_a_directory() {
    let data_dir = tempfile::tempdir().unwrap();

    fs::create_dir_all(data_dir.path().join("agent.conf"))
        .await
        .unwrap();

    let result = SpireTestCluster::start(data_dir, SpireTestClusterParams::default(), 20002).await;

    assert!(result.is_err());
}

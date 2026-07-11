use std::path::PathBuf;

use margaret_spiffe_svid_manager_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_manager_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn errors_when_agent_binary_does_not_exist() {
    let result = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams {
            agent_binary_path: PathBuf::from("/nonexistent/spire-agent-coverage-test"),
            ..Default::default()
        },
        20001,
    )
    .await;

    assert!(result.is_err());
}

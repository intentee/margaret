use std::time::Duration;

use margaret_spiffe_svid_manager_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_manager_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn errors_when_workload_api_readiness_times_out() {
    let result = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams {
            workload_api_readiness_timeout: Duration::from_millis(1),
            ..Default::default()
        },
        20012,
    )
    .await;

    assert!(result.is_err());
}

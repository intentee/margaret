use margaret_spiffe_svid_manager_tests::spire_server_real_daemon_with_failing_bundle_cli::spire_server_real_daemon_with_failing_bundle_cli;
use margaret_spiffe_svid_manager_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_manager_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn errors_when_bundle_show_cli_rejected() {
    let result = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams {
            server_binary_path: spire_server_real_daemon_with_failing_bundle_cli(),
            ..Default::default()
        },
        20004,
    )
    .await;

    assert!(result.is_err());
}

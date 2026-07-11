use margaret_spiffe_svid_manager_tests::scenario_fixture_paths::spire_server_real_daemon_with_failing_entry_cli;
use margaret_spiffe_svid_manager_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_manager_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn errors_when_entry_create_cli_rejected() {
    let result = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams {
            server_binary_path: spire_server_real_daemon_with_failing_entry_cli(),
            ..Default::default()
        },
        20006,
    )
    .await;

    assert!(result.is_err());
}

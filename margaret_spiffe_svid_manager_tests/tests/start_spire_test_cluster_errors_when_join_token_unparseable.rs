use margaret_spiffe_svid_manager_tests::spire_server_real_daemon_with_unparseable_token_cli::spire_server_real_daemon_with_unparseable_token_cli;
use margaret_spiffe_svid_manager_tests::spire_test_cluster::SpireTestCluster;
use margaret_spiffe_svid_manager_tests::spire_test_cluster_params::SpireTestClusterParams;

#[tokio::test]
async fn errors_when_token_generate_output_is_unparseable() {
    let result = SpireTestCluster::start(
        tempfile::tempdir().unwrap(),
        SpireTestClusterParams {
            server_binary_path: spire_server_real_daemon_with_unparseable_token_cli(),
            ..Default::default()
        },
        20007,
    )
    .await;

    assert!(result.is_err());
}

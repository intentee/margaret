use margaret_spiffe_svid_manager_tests::scenario_fixture_paths::spire_server_exits_immediately;
use margaret_spiffe_svid_manager_tests::spawn_test_subprocess::spawn_test_subprocess;

#[tokio::test]
async fn exits_immediately_fixture_returns_quickly() {
    let mut child = spawn_test_subprocess(&spire_server_exits_immediately(), &[])
        .await
        .unwrap();

    child.wait().await.unwrap();
}

use margaret_spiffe_svid_tests::spawn_test_subprocess::spawn_test_subprocess;
use margaret_spiffe_svid_tests::spire_server_exits_immediately::spire_server_exits_immediately;

#[tokio::test]
async fn exits_immediately_fixture_returns_quickly() {
    let mut child = spawn_test_subprocess(&spire_server_exits_immediately(), &[])
        .await
        .unwrap();

    child.wait().await.unwrap();
}

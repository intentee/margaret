use std::path::PathBuf;

use margaret_spiffe_svid_tests::run_spire_server::run_spire_server;

#[tokio::test]
async fn returns_error_for_invalid_subcommand() {
    let socket_path = PathBuf::from("/tmp/intentee-tests-nonexistent-spire.sock");

    let result = run_spire_server(&socket_path, &["definitely-not-a-real-subcommand"]).await;

    assert!(result.is_err());
}

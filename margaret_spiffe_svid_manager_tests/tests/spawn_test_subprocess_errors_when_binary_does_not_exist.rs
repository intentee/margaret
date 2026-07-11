use std::path::Path;

use margaret_spiffe_svid_manager_tests::spawn_test_subprocess::spawn_test_subprocess;

#[tokio::test]
async fn errors_when_binary_does_not_exist() {
    let result = spawn_test_subprocess(
        Path::new("/nonexistent/test-tool-spawn-coverage-binary"),
        &[],
    )
    .await;

    assert!(result.is_err());
}

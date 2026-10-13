use std::path::Path;

use margaret_spiffe_svid_tests::spawn_test_subprocess::spawn_test_subprocess;

#[tokio::test]
async fn errors_when_the_binary_does_not_exist_so_a_missing_tool_fails_the_test_setup() {
    let result = spawn_test_subprocess(
        Path::new("/nonexistent/test-tool-spawn-coverage-binary"),
        &[],
    )
    .await;

    assert!(result.is_err());
}

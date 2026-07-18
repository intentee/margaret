use std::path::Path;

use margaret_spiffe_svid_tests::spawn_test_subprocess::spawn_test_subprocess;

#[tokio::test]
async fn spawns_real_short_lived_binary() {
    let mut child = spawn_test_subprocess(Path::new("true"), &[]).await.unwrap();

    child.wait().await.unwrap();
}

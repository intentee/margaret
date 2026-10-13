use std::path::PathBuf;

use margaret_spiffe_svid_tests::spire_test_cluster_layout::SpireTestClusterLayout;

#[tokio::test]
async fn errors_when_the_server_socket_is_unreachable_instead_of_returning_a_bundle() {
    let tempdir = tempfile::tempdir().unwrap();
    let layout = SpireTestClusterLayout::new(tempdir.path(), "spiffe.test", 1234);

    let result = layout
        .obtain_trust_bundle(&PathBuf::from("spire-server"))
        .await;

    assert!(result.is_err());
}

use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test]
async fn signing_keys_start_absent() {
    let started = started_with_signing_keys().await;

    assert_eq!(
        StoredRevision::loaded(&started.database).await,
        StoredRevision::Absent
    );
}

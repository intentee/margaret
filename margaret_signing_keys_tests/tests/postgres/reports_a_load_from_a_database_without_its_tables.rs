use margaret_database_tests::started_database::StartedDatabase;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_error::SigningKeysError;

#[tokio::test]
async fn reports_a_load_from_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        SigningKeySet::load(&started.database).await,
        Err(SigningKeysError::Load(_))
    ));
}

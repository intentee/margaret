use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_replacement::SigningKeysReplacement;
use margaret_signing_keys_tests::contract_revision::contract_revision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test]
async fn signing_keys_replacement_refuses_absent_keys() {
    let started = started_with_signing_keys().await;
    let database = started.database.as_ref();

    assert_eq!(
        SigningKeySet::replace(
            database,
            SigningKeysGeneration::FIRST,
            &contract_revision(1)
        )
        .await
        .expect("the replacement of absent keys is refused"),
        SigningKeysReplacement::Superseded
    );
    assert_eq!(
        StoredRevision::loaded(database).await,
        StoredRevision::Absent
    );
}

use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_creation::SigningKeysCreation;
use margaret_signing_keys_tests::contract_revision::contract_revision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test]
async fn signing_keys_creation_refuses_existing_keys() {
    let started = started_with_signing_keys().await;
    let database = started.database.as_ref();
    let first = contract_revision(0);

    assert_eq!(
        SigningKeySet::create(database, &first)
            .await
            .expect("the signing keys are created"),
        SigningKeysCreation::Created
    );
    assert_eq!(
        SigningKeySet::create(database, &contract_revision(0))
            .await
            .expect("a second creation is refused"),
        SigningKeysCreation::AlreadyCreated
    );
    assert_eq!(
        StoredRevision::loaded(database).await,
        StoredRevision::of(&first)
    );
}

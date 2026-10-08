use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_creation::SigningKeysCreation;
use margaret_signing_keys::signing_keys_replacement::SigningKeysReplacement;
use margaret_signing_keys_tests::contract_revision::contract_revision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test]
async fn signing_keys_replacement_refuses_another_generation() {
    let started = started_with_signing_keys().await;
    let database = started.database.as_ref();
    let replacement = contract_revision(1);

    assert_eq!(
        SigningKeySet::create(database, &contract_revision(0))
            .await
            .expect("the signing keys are created"),
        SigningKeysCreation::Created
    );
    assert_eq!(
        SigningKeySet::replace(database, SigningKeysGeneration::FIRST, &replacement)
            .await
            .expect("the signing keys are replaced"),
        SigningKeysReplacement::Replaced
    );
    assert_eq!(
        SigningKeySet::replace(
            database,
            SigningKeysGeneration::FIRST,
            &contract_revision(1)
        )
        .await
        .expect("a replacement of another generation is refused"),
        SigningKeysReplacement::Superseded
    );
    assert_eq!(
        StoredRevision::loaded(database).await,
        StoredRevision::of(&replacement)
    );
}

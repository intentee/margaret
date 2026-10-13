use margaret::framework::active_record::model::Model;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_key_set_name::SIGNING_KEY_SET_NAME;
use margaret_signing_keys::signing_keys_error::SigningKeysError;
use margaret_signing_keys_tests::contract_revision::contract_revision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn reports_a_stored_generation_no_roll_produces() {
    let started = started_with_signing_keys().await;
    let database = started.database.as_ref();

    SigningKeySet::create(database, &contract_revision(0))
        .await
        .expect("the signing keys are created");
    SigningKeySet::query()
        .name
        .eq(SIGNING_KEY_SET_NAME.to_string())
        .update(database, |columns| columns.generation.to(-1))
        .await
        .expect("the stored generation is corrupted");

    assert!(matches!(
        SigningKeySet::load(database).await,
        Err(SigningKeysError::StoredGenerationOutOfRange { generation: -1, .. })
    ));
}

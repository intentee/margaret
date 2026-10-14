use margaret::framework::active_record::model::Model;
use margaret::framework::sql_identifier::qualified_table::qualified_table;
use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_signing_keys::constants::SIGNING_KEY_SET_NAME;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_error::SigningKeysError;
use margaret_signing_keys_tests::contract_revision::contract_revision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn reports_a_stored_nonce_no_seal_produces() {
    let started = started_with_signing_keys().await;
    let database = started.database.as_ref();

    SigningKeySet::create(database, &contract_revision(0))
        .await
        .expect("the signing keys are created");
    started
        .administration
        .execute(&format!(
            "ALTER TABLE {} DROP CONSTRAINT signing_key_sets_nonce_byte_length",
            qualified_table(TableNamespace::Framework, "signing_key_sets")
        ))
        .await;
    SigningKeySet::query()
        .name
        .eq(SIGNING_KEY_SET_NAME.to_string())
        .update(database, |columns| columns.nonce.to(vec![0; 11]))
        .await
        .expect("the stored nonce is corrupted");

    assert!(matches!(
        SigningKeySet::load(database).await,
        Err(SigningKeysError::StoredNonceLength { length: 11 })
    ));
}

use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_error::SigningKeysError;
use margaret_signing_keys_tests::contract_revision::contract_revision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn reports_a_creation_the_database_refuses() {
    let started = started_with_signing_keys().await;

    started
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "signing_key_sets",
        )
        .await;

    assert!(matches!(
        SigningKeySet::create(&started.database, &contract_revision(0)).await,
        Err(SigningKeysError::Create(_))
    ));
}

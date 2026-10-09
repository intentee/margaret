use std::sync::Arc;

use margaret_database_tests::started_database::StartedDatabase;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::signing_keys_revision::SigningKeysRevision;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;
use margaret_model::qualified_framework_table::qualified_framework_table;
use margaret_signing_keys_database::database_signing_keys::DatabaseSigningKeys;
use margaret_signing_keys_database::signing_keys_database_error::SigningKeysDatabaseError;
use margaret_store_contract_tests::contract_revision::contract_revision;

use crate::framework_state_database::framework_state_database;

fn beyond_the_database_range() -> SigningKeysRevision {
    SigningKeysRevision {
        generation: SigningKeysGeneration::new(u64::MAX),
        ..contract_revision(0)
    }
}

#[tokio::test]
async fn reports_a_load_from_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;
    let error = DatabaseSigningKeys::create(Arc::clone(&started.database))
        .load_signing_keys()
        .await
        .err()
        .expect("the load fails without the signing keys table");

    assert!(matches!(
        error.downcast_ref::<SigningKeysDatabaseError>(),
        Some(SigningKeysDatabaseError::Load(_))
    ));
}

#[tokio::test]
async fn reports_a_stored_generation_no_roll_produces() {
    let started = framework_state_database().await;
    let store = DatabaseSigningKeys::create(Arc::clone(&started.database));

    store
        .create_signing_keys(&contract_revision(0))
        .await
        .expect("the store creates the signing keys");
    started
        .database
        .client()
        .await
        .expect("a connection is checked out")
        .execute(
            &format!(
                "UPDATE {} SET generation = -1",
                qualified_framework_table("signing_key_sets")
            ),
            &[],
        )
        .await
        .expect("the stored generation is corrupted");

    let error = store
        .load_signing_keys()
        .await
        .err()
        .expect("the load refuses a negative generation");

    assert!(matches!(
        error.downcast_ref::<SigningKeysDatabaseError>(),
        Some(SigningKeysDatabaseError::StoredGenerationOutOfRange { generation: -1, .. })
    ));
}

#[tokio::test]
async fn refuses_to_create_a_generation_beyond_the_database_range() {
    let started = framework_state_database().await;
    let error = DatabaseSigningKeys::create(Arc::clone(&started.database))
        .create_signing_keys(&beyond_the_database_range())
        .await
        .expect_err("the creation refuses a generation beyond the database range");

    assert!(matches!(
        error.downcast_ref::<SigningKeysDatabaseError>(),
        Some(SigningKeysDatabaseError::GenerationOutOfRange { generation, .. })
            if *generation == SigningKeysGeneration::new(u64::MAX)
    ));
}

#[tokio::test]
async fn refuses_to_replace_with_a_generation_beyond_the_database_range() {
    let started = framework_state_database().await;
    let error = DatabaseSigningKeys::create(Arc::clone(&started.database))
        .replace_signing_keys(SigningKeysGeneration::FIRST, &beyond_the_database_range())
        .await
        .expect_err("the replacement refuses a generation beyond the database range");

    assert!(matches!(
        error.downcast_ref::<SigningKeysDatabaseError>(),
        Some(SigningKeysDatabaseError::GenerationOutOfRange { generation, .. })
            if *generation == SigningKeysGeneration::new(u64::MAX)
    ));
}

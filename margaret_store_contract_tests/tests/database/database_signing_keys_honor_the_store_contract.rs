use std::sync::Arc;

use margaret_signing_keys_database::database_signing_keys::DatabaseSigningKeys;
use margaret_store_contract_tests::concurrent_signing_keys_creations_admit_one::concurrent_signing_keys_creations_admit_one;
use margaret_store_contract_tests::concurrent_signing_keys_replacements_admit_one::concurrent_signing_keys_replacements_admit_one;
use margaret_store_contract_tests::signing_keys_creation_refuses_existing_keys::signing_keys_creation_refuses_existing_keys;
use margaret_store_contract_tests::signing_keys_replacement_refuses_absent_keys::signing_keys_replacement_refuses_absent_keys;
use margaret_store_contract_tests::signing_keys_replacement_refuses_another_generation::signing_keys_replacement_refuses_another_generation;
use margaret_store_contract_tests::signing_keys_start_absent::signing_keys_start_absent;

use crate::framework_state_database::framework_state_database;

#[tokio::test]
async fn signing_keys_start_absent_for_the_database_store() {
    let started = framework_state_database().await;

    signing_keys_start_absent(&DatabaseSigningKeys::create(Arc::clone(&started.database))).await;
}

#[tokio::test]
async fn concurrent_signing_keys_creations_admit_one_for_the_database_store() {
    let started = framework_state_database().await;

    concurrent_signing_keys_creations_admit_one(&DatabaseSigningKeys::create(Arc::clone(
        &started.database,
    )))
    .await;
}

#[tokio::test]
async fn signing_keys_creation_refuses_existing_keys_for_the_database_store() {
    let started = framework_state_database().await;

    signing_keys_creation_refuses_existing_keys(&DatabaseSigningKeys::create(Arc::clone(
        &started.database,
    )))
    .await;
}

#[tokio::test]
async fn concurrent_signing_keys_replacements_admit_one_for_the_database_store() {
    let started = framework_state_database().await;

    concurrent_signing_keys_replacements_admit_one(&DatabaseSigningKeys::create(Arc::clone(
        &started.database,
    )))
    .await;
}

#[tokio::test]
async fn signing_keys_replacement_refuses_another_generation_for_the_database_store() {
    let started = framework_state_database().await;

    signing_keys_replacement_refuses_another_generation(&DatabaseSigningKeys::create(Arc::clone(
        &started.database,
    )))
    .await;
}

#[tokio::test]
async fn signing_keys_replacement_refuses_absent_keys_for_the_database_store() {
    let started = framework_state_database().await;

    signing_keys_replacement_refuses_absent_keys(&DatabaseSigningKeys::create(Arc::clone(
        &started.database,
    )))
    .await;
}

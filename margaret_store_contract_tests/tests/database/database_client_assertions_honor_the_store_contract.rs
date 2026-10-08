use std::sync::Arc;

use margaret_client_assertions_database::database_client_assertions::DatabaseClientAssertions;
use margaret_store_contract_tests::client_assertions_are_first_again_once_expired::client_assertions_are_first_again_once_expired;
use margaret_store_contract_tests::client_assertions_are_remembered_per_client::client_assertions_are_remembered_per_client;
use margaret_store_contract_tests::concurrent_client_assertions_are_first_once::concurrent_client_assertions_are_first_once;

use crate::framework_state_database::framework_state_database;

#[tokio::test]
async fn concurrent_client_assertions_are_first_once_for_the_database_store() {
    let started = framework_state_database().await;

    concurrent_client_assertions_are_first_once(&DatabaseClientAssertions::create(Arc::clone(
        &started.database,
    )))
    .await;
}

#[tokio::test]
async fn client_assertions_are_remembered_per_client_for_the_database_store() {
    let started = framework_state_database().await;

    client_assertions_are_remembered_per_client(&DatabaseClientAssertions::create(Arc::clone(
        &started.database,
    )))
    .await;
}

#[tokio::test]
async fn client_assertions_are_first_again_once_expired_for_the_database_store() {
    let started = framework_state_database().await;

    client_assertions_are_first_again_once_expired(&DatabaseClientAssertions::create(Arc::clone(
        &started.database,
    )))
    .await;
}

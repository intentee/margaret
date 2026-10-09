use margaret_accepted_clients_tests::fixture_client_assertions::FixtureClientAssertions;

use margaret_store_contract_tests::client_assertions_are_first_again_once_expired::client_assertions_are_first_again_once_expired;
use margaret_store_contract_tests::client_assertions_are_remembered_per_client::client_assertions_are_remembered_per_client;
use margaret_store_contract_tests::concurrent_client_assertions_are_first_once::concurrent_client_assertions_are_first_once;

#[tokio::test]
async fn concurrent_client_assertions_are_first_once_for_the_fixture_store() {
    concurrent_client_assertions_are_first_once(&FixtureClientAssertions::default()).await;
}

#[tokio::test]
async fn client_assertions_are_remembered_per_client_for_the_fixture_store() {
    client_assertions_are_remembered_per_client(&FixtureClientAssertions::default()).await;
}

#[tokio::test]
async fn client_assertions_are_first_again_once_expired_for_the_fixture_store() {
    client_assertions_are_first_again_once_expired(&FixtureClientAssertions::default()).await;
}

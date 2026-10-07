use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::spends_the_assertions_of_distinct_clients_independently::spends_the_assertions_of_distinct_clients_independently;

#[tokio::test]
async fn memory_state_spends_the_assertions_of_distinct_clients_independently() {
    spends_the_assertions_of_distinct_clients_independently(&MemoryProviderState::create()).await;
}

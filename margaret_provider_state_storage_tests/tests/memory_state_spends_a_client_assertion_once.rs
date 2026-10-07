use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::spends_a_client_assertion_once::spends_a_client_assertion_once;

#[tokio::test]
async fn memory_state_spends_a_client_assertion_once() {
    spends_a_client_assertion_once(&MemoryProviderState::create()).await;
}

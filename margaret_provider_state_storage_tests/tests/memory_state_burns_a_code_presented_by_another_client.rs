use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::burns_a_code_presented_by_another_client::burns_a_code_presented_by_another_client;

#[tokio::test]
async fn memory_state_burns_a_code_presented_by_another_client() {
    burns_a_code_presented_by_another_client(&MemoryProviderState::create()).await;
}

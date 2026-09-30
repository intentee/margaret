use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::refuses_to_rotate_a_token_for_another_client::refuses_to_rotate_a_token_for_another_client;

#[tokio::test]
async fn memory_state_refuses_to_rotate_a_token_for_another_client() {
    refuses_to_rotate_a_token_for_another_client(&MemoryProviderState::create()).await;
}

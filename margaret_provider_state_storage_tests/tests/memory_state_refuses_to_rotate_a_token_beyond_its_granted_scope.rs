use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::refuses_to_rotate_a_token_beyond_its_granted_scope::refuses_to_rotate_a_token_beyond_its_granted_scope;

#[tokio::test]
async fn memory_state_refuses_to_rotate_a_token_beyond_its_granted_scope() {
    refuses_to_rotate_a_token_beyond_its_granted_scope(&MemoryProviderState::create()).await;
}

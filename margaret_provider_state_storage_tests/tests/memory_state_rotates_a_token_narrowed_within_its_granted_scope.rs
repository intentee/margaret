use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::rotates_a_token_narrowed_within_its_granted_scope::rotates_a_token_narrowed_within_its_granted_scope;

#[tokio::test]
async fn memory_state_rotates_a_token_narrowed_within_its_granted_scope() {
    rotates_a_token_narrowed_within_its_granted_scope(&MemoryProviderState::create()).await;
}

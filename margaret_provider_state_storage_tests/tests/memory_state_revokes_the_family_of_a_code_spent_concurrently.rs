use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::revokes_the_family_of_a_code_spent_concurrently::revokes_the_family_of_a_code_spent_concurrently;

#[tokio::test]
async fn memory_state_revokes_the_family_of_a_code_spent_concurrently() {
    revokes_the_family_of_a_code_spent_concurrently(&MemoryProviderState::create()).await;
}

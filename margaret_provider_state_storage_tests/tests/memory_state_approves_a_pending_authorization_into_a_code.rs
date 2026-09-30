use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::approves_a_pending_authorization_into_a_code::approves_a_pending_authorization_into_a_code;

#[tokio::test]
async fn memory_state_approves_a_pending_authorization_into_a_code() {
    approves_a_pending_authorization_into_a_code(&MemoryProviderState::create()).await;
}

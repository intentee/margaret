use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::rotates_a_current_refresh_token::rotates_a_current_refresh_token;

#[tokio::test]
async fn memory_state_rotates_a_current_refresh_token() {
    rotates_a_current_refresh_token(&MemoryProviderState::create()).await;
}

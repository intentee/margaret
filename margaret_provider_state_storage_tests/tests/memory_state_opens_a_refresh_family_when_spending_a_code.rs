use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::opens_a_refresh_family_when_spending_a_code::opens_a_refresh_family_when_spending_a_code;

#[tokio::test]
async fn memory_state_opens_a_refresh_family_when_spending_a_code() {
    opens_a_refresh_family_when_spending_a_code(&MemoryProviderState::create()).await;
}

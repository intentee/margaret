use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::reports_the_spending_of_an_unknown_code::reports_the_spending_of_an_unknown_code;

#[tokio::test]
async fn memory_state_reports_the_spending_of_an_unknown_code() {
    reports_the_spending_of_an_unknown_code(&MemoryProviderState::create()).await;
}

use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::reports_an_unknown_refresh_token::reports_an_unknown_refresh_token;

#[tokio::test]
async fn memory_state_reports_an_unknown_refresh_token() {
    reports_an_unknown_refresh_token(&MemoryProviderState::create()).await;
}

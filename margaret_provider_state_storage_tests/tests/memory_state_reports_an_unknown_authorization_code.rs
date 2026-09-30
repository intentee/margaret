use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::reports_an_unknown_authorization_code::reports_an_unknown_authorization_code;

#[tokio::test]
async fn memory_state_reports_an_unknown_authorization_code() {
    reports_an_unknown_authorization_code(&MemoryProviderState::create()).await;
}

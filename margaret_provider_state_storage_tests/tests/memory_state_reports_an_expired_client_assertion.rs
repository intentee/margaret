use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::reports_an_expired_client_assertion::reports_an_expired_client_assertion;

#[tokio::test]
async fn memory_state_reports_an_expired_client_assertion() {
    reports_an_expired_client_assertion(&MemoryProviderState::create()).await;
}

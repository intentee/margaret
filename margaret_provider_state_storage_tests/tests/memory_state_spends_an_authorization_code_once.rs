use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::spends_an_authorization_code_once::spends_an_authorization_code_once;

#[tokio::test]
async fn memory_state_spends_an_authorization_code_once() {
    spends_an_authorization_code_once(&MemoryProviderState::create()).await;
}

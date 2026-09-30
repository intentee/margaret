use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::redeems_an_authorization_code_once::redeems_an_authorization_code_once;

#[tokio::test]
async fn memory_state_redeems_an_authorization_code_once() {
    redeems_an_authorization_code_once(&MemoryProviderState::create()).await;
}

use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::burns_a_code_presented_with_another_challenge::burns_a_code_presented_with_another_challenge;

#[tokio::test]
async fn memory_state_burns_a_code_presented_with_another_challenge() {
    burns_a_code_presented_with_another_challenge(&MemoryProviderState::create()).await;
}

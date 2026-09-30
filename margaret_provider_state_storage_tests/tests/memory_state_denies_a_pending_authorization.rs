use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::denies_a_pending_authorization::denies_a_pending_authorization;

#[tokio::test]
async fn memory_state_denies_a_pending_authorization() {
    denies_a_pending_authorization(&MemoryProviderState::create()).await;
}

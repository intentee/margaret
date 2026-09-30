use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::revokes_the_refresh_family_of_a_replayed_code::revokes_the_refresh_family_of_a_replayed_code;

#[tokio::test]
async fn memory_state_revokes_the_refresh_family_of_a_replayed_code() {
    revokes_the_refresh_family_of_a_replayed_code(&MemoryProviderState::create()).await;
}

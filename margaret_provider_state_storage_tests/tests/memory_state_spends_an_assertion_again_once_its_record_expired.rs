use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::spends_an_assertion_again_once_its_record_expired::spends_an_assertion_again_once_its_record_expired;

#[tokio::test]
async fn memory_state_spends_an_assertion_again_once_its_record_expired() {
    spends_an_assertion_again_once_its_record_expired(&MemoryProviderState::create()).await;
}

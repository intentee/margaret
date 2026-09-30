use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage_tests::scenarios::forgets_a_pending_authorization_decided_by_another_subject::forgets_a_pending_authorization_decided_by_another_subject;

#[tokio::test]
async fn memory_state_forgets_a_pending_authorization_decided_by_another_subject() {
    forgets_a_pending_authorization_decided_by_another_subject(&MemoryProviderState::create())
        .await;
}

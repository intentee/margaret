use uuid::Uuid;

use margaret_provider_state_storage::decided_authorization::DecidedAuthorization;
use margaret_provider_state_storage::pending_decision::PendingDecision;
use margaret_provider_state_storage::pending_verdict::PendingVerdict;
use margaret_provider_state_storage_selection::provider_state_storage_uri::ProviderStateStorageUri;
use margaret_provider_state_storage_selection::resolve_provider_state_storage::resolve_provider_state_storage;

#[tokio::test]
async fn resolve_selects_memory_storage() {
    let state = resolve_provider_state_storage(ProviderStateStorageUri::Memory);

    assert_eq!(
        state
            .decide_pending_authorization(
                Uuid::new_v4(),
                PendingDecision {
                    subject: Uuid::new_v4(),
                    verdict: PendingVerdict::Denied,
                },
            )
            .await
            .expect("the memory storage decides"),
        DecidedAuthorization::Unknown
    );
}

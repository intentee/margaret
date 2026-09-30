use uuid::Uuid;

use margaret_provider_state_storage::decided_authorization::DecidedAuthorization;
use margaret_provider_state_storage::pending_decision::PendingDecision;
use margaret_provider_state_storage::pending_verdict::PendingVerdict;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::pending_of::pending_of;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn forgets_a_pending_authorization_decided_by_another_subject(
    state: &dyn StoresProviderState,
) {
    let pending = pending_of();
    let id = Uuid::new_v4();

    state
        .hold_pending_authorization(id, pending.clone())
        .await
        .expect("the backend holds the pending authorization");

    for subject in [Uuid::from_u128(99), pending.grant.subject] {
        assert_eq!(
            state
                .decide_pending_authorization(
                    id,
                    PendingDecision {
                        subject,
                        verdict: PendingVerdict::Denied,
                    },
                )
                .await
                .expect("the backend decides the pending authorization"),
            DecidedAuthorization::Unknown
        );
    }
}

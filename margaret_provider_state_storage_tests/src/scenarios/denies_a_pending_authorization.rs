use uuid::Uuid;

use margaret_provider_state_storage::decided_authorization::DecidedAuthorization;
use margaret_provider_state_storage::pending_decision::PendingDecision;
use margaret_provider_state_storage::pending_verdict::PendingVerdict;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::pending_of::pending_of;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn denies_a_pending_authorization(state: &dyn StoresProviderState) {
    let pending = pending_of();
    let id = Uuid::new_v4();
    let denial = PendingDecision {
        subject: pending.grant.subject,
        verdict: PendingVerdict::Denied,
    };

    state
        .hold_pending_authorization(id, pending.clone())
        .await
        .expect("the backend holds the pending authorization");

    assert_eq!(
        state
            .decide_pending_authorization(id, denial)
            .await
            .expect("the backend decides the pending authorization"),
        DecidedAuthorization::Denied(Box::new(pending))
    );
}

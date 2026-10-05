use uuid::Uuid;

use margaret_provider_state_storage::decided_authorization::DecidedAuthorization;
use margaret_provider_state_storage::pending_decision::PendingDecision;
use margaret_provider_state_storage::pending_verdict::PendingVerdict;
use margaret_provider_state_storage::presented_code::PresentedCode;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fresh_digest::fresh_digest;
use crate::pending_of::pending_of;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn approves_a_pending_authorization_into_a_code(state: &dyn StoresProviderState) {
    let pending = pending_of();
    let id = Uuid::new_v4();
    let code = fresh_digest();
    let approval = PendingDecision {
        subject: pending.grant.subject,
        verdict: PendingVerdict::Approved { code },
    };

    state
        .hold_pending_authorization(id, pending.clone())
        .await
        .expect("the backend holds the pending authorization");

    assert_eq!(
        state
            .decide_pending_authorization(id, approval)
            .await
            .expect("the backend decides the pending authorization"),
        DecidedAuthorization::Approved(Box::new(pending.clone()))
    );
    assert_eq!(
        state
            .decide_pending_authorization(id, approval)
            .await
            .expect("the backend decides the pending authorization"),
        DecidedAuthorization::Unknown
    );
    assert_eq!(
        state
            .present_code(code)
            .await
            .expect("the backend looks up the code"),
        PresentedCode::Issued(Box::new(pending.grant))
    );
}

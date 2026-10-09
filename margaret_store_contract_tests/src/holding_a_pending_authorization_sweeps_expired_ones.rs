use uuid::Uuid;

use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_authorization_grants::pending_authorization_take::PendingAuthorizationTake;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::contract_instant::contract_instant;
use crate::contract_pending_authorization::contract_pending_authorization;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn holding_a_pending_authorization_sweeps_expired_ones(
    store: &dyn StoresAuthorizationGrants,
) {
    let now = contract_instant();
    let expired = Uuid::new_v4();

    store
        .hold_pending_authorization(
            expired,
            PendingAuthorization {
                expires_at: now,
                ..contract_pending_authorization()
            },
            now,
        )
        .await
        .expect("the store holds the expiring pending authorization");
    store
        .hold_pending_authorization(Uuid::new_v4(), contract_pending_authorization(), now)
        .await
        .expect("the store holds another pending authorization");

    assert_eq!(
        store
            .take_pending_authorization(expired)
            .await
            .expect("the store takes pending authorizations"),
        PendingAuthorizationTake::Absent
    );
}

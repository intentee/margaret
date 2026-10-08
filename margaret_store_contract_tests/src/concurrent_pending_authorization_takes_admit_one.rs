use futures_util::future::join_all;
use uuid::Uuid;

use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_authorization_grants::pending_authorization_take::PendingAuthorizationTake;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::contract_pending_authorization::contract_pending_authorization;
use crate::racing_instances::RACING_INSTANCES;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn concurrent_pending_authorization_takes_admit_one(
    store: &dyn StoresAuthorizationGrants,
) {
    let id = Uuid::new_v4();
    let pending = contract_pending_authorization();

    store
        .hold_pending_authorization(id, pending.clone())
        .await
        .expect("the store holds the pending authorization");

    let taken: Vec<PendingAuthorization> = join_all((0..RACING_INSTANCES).map(|_| async move {
        match store
            .take_pending_authorization(id)
            .await
            .expect("the store takes the pending authorization")
        {
            PendingAuthorizationTake::Absent => None,
            PendingAuthorizationTake::Taken(taken) => Some(*taken),
        }
    }))
    .await
    .into_iter()
    .flatten()
    .collect();

    assert_eq!(taken, [pending]);
}

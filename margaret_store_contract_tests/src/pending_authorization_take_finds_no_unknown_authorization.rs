use uuid::Uuid;

use margaret_authorization_grants::pending_authorization_take::PendingAuthorizationTake;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn pending_authorization_take_finds_no_unknown_authorization(
    store: &dyn StoresAuthorizationGrants,
) {
    assert_eq!(
        store
            .take_pending_authorization(Uuid::new_v4())
            .await
            .expect("the store takes pending authorizations"),
        PendingAuthorizationTake::Absent
    );
}

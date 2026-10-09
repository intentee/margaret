use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_family_lifetime::REFRESH_FAMILY_LIFETIME;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::contract_instant::contract_instant;
use crate::contract_refresh_family::contract_refresh_family;
use crate::contract_token::contract_token;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn opening_a_refresh_family_sweeps_expired_revocations(
    store: &dyn StoresAuthorizationGrants,
) {
    let revoked_at = contract_instant();
    let family = Uuid::new_v4();
    let expired_at = revoked_at.after(REFRESH_FAMILY_LIFETIME);

    store
        .revoke_refresh_family(family, revoked_at)
        .await
        .expect("the store revokes the unopened family");

    assert_eq!(
        store
            .open_refresh_family(
                family,
                contract_refresh_family(),
                contract_token(),
                expired_at
            )
            .await
            .expect("the store opens refresh families"),
        FamilyOpening::Opened
    );
}

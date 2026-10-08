use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::contract_refresh_family::contract_refresh_family;
use crate::contract_token::contract_token;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn revoked_refresh_family_neither_resolves_nor_rotates(
    store: &dyn StoresAuthorizationGrants,
) {
    let family = Uuid::new_v4();
    let presented = contract_token();
    let next = contract_token();

    assert_eq!(
        store
            .open_refresh_family(family, contract_refresh_family(), presented)
            .await
            .expect("the store opens the refresh family"),
        FamilyOpening::Opened
    );

    store
        .revoke_refresh_family(family)
        .await
        .expect("the store revokes the refresh family");

    assert_eq!(
        store
            .find_refresh_token(presented)
            .await
            .expect("the store finds refresh tokens"),
        RefreshTokenLookup::Unknown
    );
    assert_eq!(
        store
            .rotate_refresh_token(presented, next)
            .await
            .expect("the store rotates refresh tokens"),
        RefreshRotation::Unknown
    );
    assert_eq!(
        store
            .find_refresh_token(next)
            .await
            .expect("the store finds refresh tokens"),
        RefreshTokenLookup::Unknown
    );
}

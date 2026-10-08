use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::contract_refresh_family::contract_refresh_family;
use crate::contract_token::contract_token;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn refresh_family_revoked_before_opening_never_opens(
    store: &dyn StoresAuthorizationGrants,
) {
    let family = Uuid::new_v4();
    let first_token = contract_token();

    store
        .revoke_refresh_family(family)
        .await
        .expect("the store revokes the refresh family");

    assert_eq!(
        store
            .open_refresh_family(family, contract_refresh_family(), first_token)
            .await
            .expect("the store opens refresh families"),
        FamilyOpening::Revoked
    );
    assert_eq!(
        store
            .find_refresh_token(first_token)
            .await
            .expect("the store finds refresh tokens"),
        RefreshTokenLookup::Unknown
    );
}

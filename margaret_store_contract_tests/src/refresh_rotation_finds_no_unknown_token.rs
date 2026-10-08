use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::contract_token::contract_token;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn refresh_rotation_finds_no_unknown_token(store: &dyn StoresAuthorizationGrants) {
    assert_eq!(
        store
            .rotate_refresh_token(contract_token(), contract_token())
            .await
            .expect("the store rotates refresh tokens"),
        RefreshRotation::Unknown
    );
}

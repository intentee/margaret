use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::contract_refresh_family::contract_refresh_family;
use crate::contract_token::contract_token;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn opened_refresh_family_resolves_its_first_token(store: &dyn StoresAuthorizationGrants) {
    let family = Uuid::new_v4();
    let record = contract_refresh_family();
    let first_token = contract_token();

    assert_eq!(
        store
            .open_refresh_family(family, record.clone(), first_token)
            .await
            .expect("the store opens the refresh family"),
        FamilyOpening::Opened
    );
    assert_eq!(
        store
            .find_refresh_token(first_token)
            .await
            .expect("the store finds refresh tokens"),
        RefreshTokenLookup::Current { family, record }
    );
}

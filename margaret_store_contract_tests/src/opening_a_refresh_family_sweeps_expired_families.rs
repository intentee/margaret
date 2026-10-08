use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::contract_instant::contract_instant;
use crate::contract_refresh_family::contract_refresh_family;
use crate::contract_token::contract_token;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn opening_a_refresh_family_sweeps_expired_families(
    store: &dyn StoresAuthorizationGrants,
) {
    let now = contract_instant();
    let expired_token = contract_token();

    assert_eq!(
        store
            .open_refresh_family(
                Uuid::new_v4(),
                RefreshFamily {
                    expires_at: now,
                    ..contract_refresh_family()
                },
                expired_token,
                now,
            )
            .await
            .expect("the store opens the expiring refresh family"),
        FamilyOpening::Opened
    );
    assert_eq!(
        store
            .open_refresh_family(
                Uuid::new_v4(),
                contract_refresh_family(),
                contract_token(),
                now
            )
            .await
            .expect("the store opens another refresh family"),
        FamilyOpening::Opened
    );
    assert_eq!(
        store
            .find_refresh_token(expired_token)
            .await
            .expect("the store finds refresh tokens"),
        RefreshTokenLookup::Unknown
    );
}

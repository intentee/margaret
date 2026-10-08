use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_family_lifetime::REFRESH_FAMILY_LIFETIME;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::contract_instant::contract_instant;
use crate::contract_refresh_family::contract_refresh_family;
use crate::contract_token::contract_token;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn revoking_an_unopened_family_bars_it_for_the_refresh_family_lifetime(
    store: &dyn StoresAuthorizationGrants,
) {
    let revoked_at = contract_instant();
    let family = Uuid::new_v4();
    let last_moment = NumericDate::new(
        revoked_at
            .after(REFRESH_FAMILY_LIFETIME)
            .seconds_since_epoch()
            - 1,
    );

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
                last_moment
            )
            .await
            .expect("the store opens refresh families"),
        FamilyOpening::Revoked
    );
}

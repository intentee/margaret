use uuid::Uuid;

use margaret_provider_state_storage::code_redemption::CodeRedemption;
use margaret_provider_state_storage::code_redemption_request::CodeRedemptionRequest;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::refresh_rotation::RefreshRotation;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::admission_of::admission_of;
use crate::fixture_grant::fixture_grant;
use crate::fresh_digest::fresh_digest;
use crate::granted_refresh::granted_refresh;
use crate::open_family::open_family;
use crate::opened_family::OpenedFamily;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn revokes_the_refresh_family_of_a_replayed_code(state: &dyn StoresProviderState) {
    let OpenedFamily {
        code,
        family,
        token,
    } = open_family(state).await;
    let grant = fixture_grant();

    assert_eq!(
        state
            .redeem_code(
                code,
                CodeRedemptionRequest {
                    admission: admission_of(&grant),
                    family: Uuid::new_v4(),
                    refresh: RefreshIssuance::Withheld,
                },
            )
            .await
            .expect("the backend redeems the code"),
        CodeRedemption::Replayed
    );
    assert_eq!(
        state
            .rotate_refresh_token(token, fresh_digest(), granted_refresh(&family))
            .await
            .expect("the backend rotates the token"),
        RefreshRotation::Unknown
    );
}

use uuid::Uuid;

use margaret_provider_state_storage::code_redemption::CodeRedemption;
use margaret_provider_state_storage::code_redemption_request::CodeRedemptionRequest;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::admission_of::admission_of;
use crate::fixture_grant::fixture_grant;
use crate::fresh_digest::fresh_digest;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn reports_an_unknown_authorization_code(state: &dyn StoresProviderState) {
    let grant = fixture_grant();

    assert_eq!(
        state
            .redeem_code(
                fresh_digest(),
                CodeRedemptionRequest {
                    admission: admission_of(&grant),
                    family: Uuid::new_v4(),
                    refresh: RefreshIssuance::Withheld,
                },
            )
            .await
            .expect("the backend redeems the code"),
        CodeRedemption::Unknown
    );
}

use uuid::Uuid;

use margaret_provider_state_storage::code_admission::CodeAdmission;
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
pub async fn burns_a_code_presented_by_another_client(state: &dyn StoresProviderState) {
    let grant = fixture_grant();
    let code = fresh_digest();

    state
        .issue_code(code, grant.clone())
        .await
        .expect("the backend stores the code");

    assert_eq!(
        state
            .redeem_code(
                code,
                CodeRedemptionRequest {
                    admission: CodeAdmission {
                        client_id: &"other".parse().expect("the client identifier is visible"),
                        ..admission_of(&grant)
                    },
                    family: Uuid::new_v4(),
                    refresh: RefreshIssuance::Withheld,
                },
            )
            .await
            .expect("the backend redeems the code"),
        CodeRedemption::Refused
    );
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
}

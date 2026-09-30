use uuid::Uuid;

use margaret_provider_state_storage::code_redemption::CodeRedemption;
use margaret_provider_state_storage::code_redemption_request::CodeRedemptionRequest;
use margaret_provider_state_storage::refresh_family::RefreshFamily;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::admission_of::admission_of;
use crate::fixture_grant::fixture_grant;
use crate::fresh_digest::fresh_digest;
use crate::opened_family::OpenedFamily;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub async fn open_family(state: &dyn StoresProviderState) -> OpenedFamily {
    let grant = fixture_grant();
    let code = fresh_digest();
    let id = Uuid::new_v4();
    let token = fresh_digest();

    state
        .issue_code(code, grant.clone())
        .await
        .expect("the backend stores the code");

    assert_eq!(
        state
            .redeem_code(
                code,
                CodeRedemptionRequest {
                    admission: admission_of(&grant),
                    family: id,
                    refresh: RefreshIssuance::Opened(token),
                },
            )
            .await
            .expect("the backend redeems the code"),
        CodeRedemption::Redeemed(Box::new(grant.clone()))
    );

    OpenedFamily {
        code,
        family: RefreshFamily {
            auth_time: grant.auth_time,
            client_id: grant.client_id,
            id,
            scopes: grant.scopes,
            subject: grant.subject,
        },
        token,
    }
}

use margaret_provider_state_storage::code_spending::CodeSpending;
use margaret_provider_state_storage::refresh_family::RefreshFamily;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fixture_grant::fixture_grant;
use crate::fresh_digest::fresh_digest;
use crate::opened_family::OpenedFamily;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub async fn open_family(state: &dyn StoresProviderState) -> OpenedFamily {
    let grant = fixture_grant();
    let code = fresh_digest();
    let token = fresh_digest();

    state
        .issue_code(code, grant.clone())
        .await
        .expect("the backend stores the code");

    assert_eq!(
        state
            .spend_code(code, RefreshIssuance::Opened(token))
            .await
            .expect("the backend spends the code"),
        CodeSpending::Spent
    );

    OpenedFamily {
        code,
        family: RefreshFamily::opened_by(&grant),
        token,
    }
}

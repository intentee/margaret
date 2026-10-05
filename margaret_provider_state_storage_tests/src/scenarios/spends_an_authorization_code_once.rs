use margaret_provider_state_storage::code_spending::CodeSpending;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fixture_grant::fixture_grant;
use crate::fresh_digest::fresh_digest;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn spends_an_authorization_code_once(state: &dyn StoresProviderState) {
    let code = fresh_digest();
    let spend = || state.spend_code(code, RefreshIssuance::Withheld);

    state
        .issue_code(code, fixture_grant())
        .await
        .expect("the backend stores the code");

    assert_eq!(
        spend().await.expect("the backend spends the code"),
        CodeSpending::Spent
    );
    assert_eq!(
        spend().await.expect("the backend spends the code"),
        CodeSpending::Replayed
    );
}

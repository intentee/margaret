use tokio::join;

use margaret_provider_state_storage::code_spending::CodeSpending;
use margaret_provider_state_storage::presented_refresh_token::PresentedRefreshToken;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fixture_grant::fixture_grant;
use crate::fresh_digest::fresh_digest;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn revokes_the_family_of_a_code_spent_concurrently(state: &dyn StoresProviderState) {
    let code = fresh_digest();
    let first_token = fresh_digest();
    let second_token = fresh_digest();

    state
        .issue_code(code, fixture_grant())
        .await
        .expect("the backend stores the code");

    let (first, second) = join!(
        state.spend_code(code, RefreshIssuance::Opened(first_token)),
        state.spend_code(code, RefreshIssuance::Opened(second_token)),
    );
    let spendings = [
        first.expect("the backend spends the code"),
        second.expect("the backend spends the code"),
    ];

    assert!(spendings.contains(&CodeSpending::Spent));
    assert!(spendings.contains(&CodeSpending::Replayed));
    for token in [first_token, second_token] {
        assert_eq!(
            state
                .present_refresh_token(token)
                .await
                .expect("the backend looks up the token"),
            PresentedRefreshToken::Unknown
        );
    }
}

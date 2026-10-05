use margaret_provider_state_storage::code_spending::CodeSpending;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fresh_digest::fresh_digest;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn reports_the_spending_of_an_unknown_code(state: &dyn StoresProviderState) {
    assert_eq!(
        state
            .spend_code(fresh_digest(), RefreshIssuance::Withheld)
            .await
            .expect("the backend spends the code"),
        CodeSpending::Expired
    );
}

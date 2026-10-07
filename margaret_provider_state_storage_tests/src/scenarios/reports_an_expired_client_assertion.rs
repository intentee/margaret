use margaret_provider_state_storage::assertion_refusal::AssertionRefusal;
use margaret_provider_state_storage::assertion_spending::AssertionSpending;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::assertion_clock::AssertionClock;
use crate::fresh_digest::fresh_digest;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn reports_an_expired_client_assertion(state: &dyn StoresProviderState) {
    let clock = AssertionClock::start();

    assert_eq!(
        state
            .spend_client_assertion("portal", fresh_digest(), clock.now, clock.now)
            .await
            .expect("the backend judges the assertion"),
        AssertionSpending::Refused(AssertionRefusal::Expired)
    );
}

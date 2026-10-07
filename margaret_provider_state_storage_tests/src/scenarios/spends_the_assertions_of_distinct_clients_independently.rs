use margaret_provider_state_storage::assertion_spending::AssertionSpending;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::assertion_clock::AssertionClock;
use crate::fresh_digest::fresh_digest;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn spends_the_assertions_of_distinct_clients_independently(
    state: &dyn StoresProviderState,
) {
    let clock = AssertionClock::start();
    let assertion = fresh_digest();

    for client_id in ["portal", "service"] {
        assert_eq!(
            state
                .spend_client_assertion(client_id, assertion, clock.in_seconds(60), clock.now)
                .await
                .expect("the backend spends the assertion"),
            AssertionSpending::Spent
        );
    }
}

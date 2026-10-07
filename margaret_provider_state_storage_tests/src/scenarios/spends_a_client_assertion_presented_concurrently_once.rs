use tokio::join;

use margaret_provider_state_storage::assertion_refusal::AssertionRefusal;
use margaret_provider_state_storage::assertion_spending::AssertionSpending;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::assertion_clock::AssertionClock;
use crate::fresh_digest::fresh_digest;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn spends_a_client_assertion_presented_concurrently_once(
    state: &dyn StoresProviderState,
) {
    let clock = AssertionClock::start();
    let assertion = fresh_digest();
    let spend =
        || state.spend_client_assertion("portal", assertion, clock.in_seconds(60), clock.now);
    let (first, second) = join!(spend(), spend());
    let spendings = [
        first.expect("the backend spends the assertion"),
        second.expect("the backend spends the assertion"),
    ];

    assert!(spendings.contains(&AssertionSpending::Spent));
    assert!(spendings.contains(&AssertionSpending::Refused(AssertionRefusal::Replayed)));
}

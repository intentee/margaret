use margaret_provider_state_storage::assertion_refusal::AssertionRefusal;
use margaret_provider_state_storage::assertion_spending::AssertionSpending;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::assertion_clock::AssertionClock;
use crate::fresh_digest::fresh_digest;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn spends_an_assertion_again_once_its_record_expired(state: &dyn StoresProviderState) {
    let clock = AssertionClock::start();
    let assertion = fresh_digest();

    assert_eq!(
        state
            .spend_client_assertion("portal", assertion, clock.in_seconds(60), clock.now)
            .await
            .expect("the backend spends the assertion"),
        AssertionSpending::Spent
    );
    assert_eq!(
        state
            .spend_client_assertion(
                "portal",
                assertion,
                clock.in_seconds(180),
                clock.in_seconds(120)
            )
            .await
            .expect("the backend spends the assertion"),
        AssertionSpending::Spent
    );
    assert_eq!(
        state
            .spend_client_assertion(
                "portal",
                assertion,
                clock.in_seconds(180),
                clock.in_seconds(121)
            )
            .await
            .expect("the backend spends the assertion"),
        AssertionSpending::Refused(AssertionRefusal::Replayed)
    );
}

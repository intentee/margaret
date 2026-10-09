use futures_util::future::join_all;

use margaret_accepted_clients::assertion_memory::AssertionMemory;
use margaret_accepted_clients::remembers_client_assertions::RemembersClientAssertions;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::contract_assertion_expiry::contract_assertion_expiry;
use crate::contract_moment::contract_moment;
use crate::contract_token::contract_token;
use crate::racing_instances::RACING_INSTANCES;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn concurrent_client_assertions_are_first_once(store: &dyn RemembersClientAssertions) {
    let assertion = contract_token();
    let expires_at = contract_assertion_expiry();
    let now = NumericDate::from(contract_moment());
    let memories: Vec<AssertionMemory> = join_all((0..RACING_INSTANCES).map(|_| async move {
        store
            .remember_client_assertion("contract-client", assertion, expires_at, now)
            .await
            .expect("the store remembers the client assertion")
    }))
    .await;

    assert_eq!(
        memories
            .iter()
            .filter(|memory| **memory == AssertionMemory::First)
            .count(),
        1
    );
}

use margaret_accepted_clients::assertion_memory::AssertionMemory;
use margaret_accepted_clients::remembers_client_assertions::RemembersClientAssertions;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::contract_assertion_expiry::contract_assertion_expiry;
use crate::contract_moment::contract_moment;
use crate::contract_token::contract_token;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn client_assertions_are_remembered_per_client(store: &dyn RemembersClientAssertions) {
    let assertion = contract_token();
    let expires_at = contract_assertion_expiry();
    let now = NumericDate::from(contract_moment());

    for client_id in ["contract-client", "another-contract-client"] {
        assert_eq!(
            store
                .remember_client_assertion(client_id, assertion, expires_at, now)
                .await
                .expect("the store remembers the client assertion"),
            AssertionMemory::First
        );
    }
}

use margaret_accepted_clients::assertion_memory::AssertionMemory;
use margaret_accepted_clients::remembers_client_assertions::RemembersClientAssertions;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::contract_assertion_expiry::contract_assertion_expiry;
use crate::contract_moment::contract_moment;
use crate::contract_token::contract_token;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn client_assertions_are_first_again_once_expired(store: &dyn RemembersClientAssertions) {
    let assertion = contract_token();
    let expires_at = contract_assertion_expiry();
    let remembered_at = |now: NumericDate| async move {
        store
            .remember_client_assertion("contract-client", assertion, expires_at, now)
            .await
            .expect("the store remembers the client assertion")
    };

    assert_eq!(
        remembered_at(NumericDate::from(contract_moment())).await,
        AssertionMemory::First
    );
    assert_eq!(
        remembered_at(NumericDate::new(expires_at.seconds_since_epoch() - 1)).await,
        AssertionMemory::Seen
    );
    assert_eq!(remembered_at(expires_at).await, AssertionMemory::First);
}

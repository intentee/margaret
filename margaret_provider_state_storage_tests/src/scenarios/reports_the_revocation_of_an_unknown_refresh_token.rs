use margaret_provider_state_storage::refresh_revocation::RefreshRevocation;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fixture_client_id::fixture_client_id;
use crate::fresh_digest::fresh_digest;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn reports_the_revocation_of_an_unknown_refresh_token(state: &dyn StoresProviderState) {
    assert_eq!(
        state
            .revoke_refresh_token(fresh_digest(), &fixture_client_id())
            .await
            .expect("the backend revokes the token"),
        RefreshRevocation::Unknown
    );
}

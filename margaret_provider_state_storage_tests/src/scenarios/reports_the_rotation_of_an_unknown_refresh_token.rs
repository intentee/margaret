use margaret_provider_state_storage::refresh_rotation::RefreshRotation;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fresh_digest::fresh_digest;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn reports_the_rotation_of_an_unknown_refresh_token(state: &dyn StoresProviderState) {
    assert_eq!(
        state
            .rotate_refresh_token(fresh_digest(), fresh_digest())
            .await
            .expect("the backend rotates the token"),
        RefreshRotation::Revoked
    );
}

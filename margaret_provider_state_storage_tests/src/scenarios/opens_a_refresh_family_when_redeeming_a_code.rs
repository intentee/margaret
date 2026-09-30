use margaret_provider_state_storage::refresh_rotation::RefreshRotation;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fresh_digest::fresh_digest;
use crate::granted_refresh::granted_refresh;
use crate::open_family::open_family;
use crate::opened_family::OpenedFamily;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn opens_a_refresh_family_when_redeeming_a_code(state: &dyn StoresProviderState) {
    let OpenedFamily { family, token, .. } = open_family(state).await;

    assert_eq!(
        state
            .rotate_refresh_token(token, fresh_digest(), granted_refresh(&family))
            .await
            .expect("the backend rotates the token"),
        RefreshRotation::Rotated(family.clone())
    );
}

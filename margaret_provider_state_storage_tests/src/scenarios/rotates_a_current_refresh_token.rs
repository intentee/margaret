use margaret_provider_state_storage::presented_refresh_token::PresentedRefreshToken;
use margaret_provider_state_storage::refresh_rotation::RefreshRotation;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fresh_digest::fresh_digest;
use crate::open_family::open_family;
use crate::opened_family::OpenedFamily;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn rotates_a_current_refresh_token(state: &dyn StoresProviderState) {
    let OpenedFamily { family, token, .. } = open_family(state).await;
    let next = fresh_digest();

    assert_eq!(
        state
            .rotate_refresh_token(token, next)
            .await
            .expect("the backend rotates the token"),
        RefreshRotation::Rotated
    );
    assert_eq!(
        state
            .present_refresh_token(next)
            .await
            .expect("the backend looks up the token"),
        PresentedRefreshToken::Current(family)
    );
}

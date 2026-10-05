use tokio::join;

use margaret_provider_state_storage::presented_refresh_token::PresentedRefreshToken;
use margaret_provider_state_storage::refresh_rotation::RefreshRotation;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fresh_digest::fresh_digest;
use crate::open_family::open_family;
use crate::opened_family::OpenedFamily;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn revokes_the_family_of_a_refresh_token_rotated_concurrently(
    state: &dyn StoresProviderState,
) {
    let OpenedFamily { token, .. } = open_family(state).await;
    let first_next = fresh_digest();
    let second_next = fresh_digest();
    let (first, second) = join!(
        state.rotate_refresh_token(token, first_next),
        state.rotate_refresh_token(token, second_next),
    );
    let rotations = [
        first.expect("the backend rotates the token"),
        second.expect("the backend rotates the token"),
    ];

    assert!(rotations.contains(&RefreshRotation::Rotated));
    assert!(rotations.contains(&RefreshRotation::Replayed));
    for next in [first_next, second_next] {
        assert_eq!(
            state
                .present_refresh_token(next)
                .await
                .expect("the backend looks up the token"),
            PresentedRefreshToken::Unknown
        );
    }
}

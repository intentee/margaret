use margaret_provider_state_storage::refresh_revocation::RefreshRevocation;
use margaret_provider_state_storage::refresh_rotation::RefreshRotation;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fresh_digest::fresh_digest;
use crate::granted_refresh::granted_refresh;
use crate::open_family::open_family;
use crate::opened_family::OpenedFamily;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn refuses_to_revoke_a_refresh_token_of_another_client(state: &dyn StoresProviderState) {
    let OpenedFamily { family, token, .. } = open_family(state).await;

    assert_eq!(
        state
            .revoke_refresh_token(
                token,
                &"other".parse().expect("the client identifier is visible"),
            )
            .await
            .expect("the backend revokes the token"),
        RefreshRevocation::ForeignClient
    );
    assert_eq!(
        state
            .rotate_refresh_token(token, fresh_digest(), granted_refresh(&family))
            .await
            .expect("the backend rotates the token"),
        RefreshRotation::Rotated(family.clone())
    );
}

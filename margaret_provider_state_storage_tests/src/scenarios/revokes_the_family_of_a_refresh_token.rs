use margaret_provider_state_storage::presented_refresh_token::PresentedRefreshToken;
use margaret_provider_state_storage::refresh_revocation::RefreshRevocation;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::open_family::open_family;
use crate::opened_family::OpenedFamily;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn revokes_the_family_of_a_refresh_token(state: &dyn StoresProviderState) {
    let OpenedFamily { family, token, .. } = open_family(state).await;
    let revoke = || state.revoke_refresh_token(token, &family.client_id);

    assert_eq!(
        revoke().await.expect("the backend revokes the token"),
        RefreshRevocation::Revoked
    );
    assert_eq!(
        revoke().await.expect("the backend revokes the token"),
        RefreshRevocation::Unknown
    );
    assert_eq!(
        state
            .present_refresh_token(token)
            .await
            .expect("the backend looks up the token"),
        PresentedRefreshToken::Unknown
    );
}

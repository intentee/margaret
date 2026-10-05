use margaret_provider_state_storage::presented_code::PresentedCode;
use margaret_provider_state_storage::presented_refresh_token::PresentedRefreshToken;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::open_family::open_family;
use crate::opened_family::OpenedFamily;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn revokes_the_refresh_family_of_a_replayed_code(state: &dyn StoresProviderState) {
    let OpenedFamily { code, token, .. } = open_family(state).await;

    assert_eq!(
        state
            .present_code(code)
            .await
            .expect("the backend looks up the code"),
        PresentedCode::Replayed
    );
    assert_eq!(
        state
            .present_refresh_token(token)
            .await
            .expect("the backend looks up the token"),
        PresentedRefreshToken::Unknown
    );
}

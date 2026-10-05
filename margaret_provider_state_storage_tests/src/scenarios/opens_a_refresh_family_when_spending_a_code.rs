use margaret_provider_state_storage::presented_refresh_token::PresentedRefreshToken;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::open_family::open_family;
use crate::opened_family::OpenedFamily;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn opens_a_refresh_family_when_spending_a_code(state: &dyn StoresProviderState) {
    let OpenedFamily { family, token, .. } = open_family(state).await;

    assert_eq!(
        state
            .present_refresh_token(token)
            .await
            .expect("the backend looks up the token"),
        PresentedRefreshToken::Current(family)
    );
}

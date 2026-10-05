use margaret_provider_state_storage::presented_code::PresentedCode;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fresh_digest::fresh_digest;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn reports_an_unknown_authorization_code(state: &dyn StoresProviderState) {
    assert_eq!(
        state
            .present_code(fresh_digest())
            .await
            .expect("the backend looks up the code"),
        PresentedCode::Unknown
    );
}

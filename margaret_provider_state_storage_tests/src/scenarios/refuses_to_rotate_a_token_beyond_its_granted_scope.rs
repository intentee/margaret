use std::collections::BTreeSet;

use margaret_provider_state_storage::refresh_admission::RefreshAdmission;
use margaret_provider_state_storage::refresh_rotation::RefreshRotation;
use margaret_provider_state_storage::refresh_scope::RefreshScope;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fresh_digest::fresh_digest;
use crate::granted_refresh::granted_refresh;
use crate::open_family::open_family;
use crate::opened_family::OpenedFamily;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn refuses_to_rotate_a_token_beyond_its_granted_scope(state: &dyn StoresProviderState) {
    let OpenedFamily { family, token, .. } = open_family(state).await;
    let exceeding = RefreshScope::Narrowed(BTreeSet::from([
        "openid".parse().expect("the scope is a scope token"),
        "profile".parse().expect("the scope is a scope token"),
    ]));

    assert_eq!(
        state
            .rotate_refresh_token(
                token,
                fresh_digest(),
                RefreshAdmission {
                    client_id: &family.client_id,
                    scope: &exceeding,
                },
            )
            .await
            .expect("the backend rotates the token"),
        RefreshRotation::ScopeExceeded
    );
    assert_eq!(
        state
            .rotate_refresh_token(token, fresh_digest(), granted_refresh(&family))
            .await
            .expect("the backend rotates the token"),
        RefreshRotation::Rotated(family.clone())
    );
}

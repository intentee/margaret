use std::collections::BTreeSet;

use margaret_provider_state_storage::refresh_admission::RefreshAdmission;
use margaret_provider_state_storage::refresh_rotation::RefreshRotation;
use margaret_provider_state_storage::refresh_scope::RefreshScope;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::fresh_digest::fresh_digest;
use crate::open_family::open_family;
use crate::opened_family::OpenedFamily;

/// # Panics
///
/// Panics when the state storage breaks its contract.
pub async fn rotates_a_token_narrowed_within_its_granted_scope(state: &dyn StoresProviderState) {
    let OpenedFamily { family, token, .. } = open_family(state).await;
    let narrowed = RefreshScope::Narrowed(BTreeSet::from(["openid"
        .parse()
        .expect("the scope is a scope token")]));

    assert_eq!(
        state
            .rotate_refresh_token(
                token,
                fresh_digest(),
                RefreshAdmission {
                    client_id: &family.client_id,
                    scope: &narrowed,
                },
            )
            .await
            .expect("the backend rotates the token"),
        RefreshRotation::Rotated(family.clone())
    );
}

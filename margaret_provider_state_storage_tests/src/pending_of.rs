use margaret_provider_state_storage::pending_authorization::PendingAuthorization;

use crate::fixture_grant::fixture_grant;

#[must_use]
pub fn pending_of() -> PendingAuthorization {
    PendingAuthorization {
        grant: fixture_grant(),
        state: Some("af0ifjsldkj".to_string()),
    }
}

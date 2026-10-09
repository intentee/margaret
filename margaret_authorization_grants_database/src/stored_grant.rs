use margaret_authorization_grants::authorization_grant::AuthorizationGrant;

use crate::authorization_grants_database_error::AuthorizationGrantsDatabaseError;

pub(crate) fn stored_grant(
    grant: &str,
) -> Result<AuthorizationGrant, AuthorizationGrantsDatabaseError> {
    serde_json::from_str(grant).map_err(AuthorizationGrantsDatabaseError::MalformedGrant)
}

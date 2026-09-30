use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

use margaret_issuer_request::issuer_request_timeout::ISSUER_REQUEST_TIMEOUT;
use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[derive(Serialize)]
pub(crate) struct ClientAssertionClaims<'claims> {
    aud: &'claims str,
    exp: i64,
    iat: i64,
    iss: &'claims str,
    jti: Uuid,
    sub: &'claims str,
}

impl<'claims> ClientAssertionClaims<'claims> {
    pub(crate) fn new(
        client_id: &'claims ClientId,
        issuer: &'claims IssuerIdentifier,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            aud: issuer.as_str(),
            exp: now
                .timestamp()
                .saturating_add(ISSUER_REQUEST_TIMEOUT.as_secs().cast_signed()),
            iat: now.timestamp(),
            iss: client_id.as_str(),
            jti: Uuid::new_v4(),
            sub: client_id.as_str(),
        }
    }
}

use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret_identity_session::session_access_token_claims::SessionAccessTokenClaims;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Session {
    pub authenticated_at: DateTime<Utc>,
    pub id: Uuid,
    pub subject: Uuid,
}

impl From<SessionAccessTokenClaims> for Session {
    fn from(
        SessionAccessTokenClaims {
            auth_time,
            sid,
            sub,
        }: SessionAccessTokenClaims,
    ) -> Self {
        Self {
            authenticated_at: auth_time,
            id: sid,
            subject: sub,
        }
    }
}

use chrono::SubsecRound as _;
use chrono::Utc;
use uuid::Uuid;

use margaret_identity_session::session_access_token_claims::SessionAccessTokenClaims;

#[must_use]
pub fn fixture_session_claims() -> SessionAccessTokenClaims {
    SessionAccessTokenClaims {
        auth_time: Utc::now().trunc_subsecs(0),
        sid: Uuid::new_v4(),
        sub: Uuid::new_v4(),
    }
}

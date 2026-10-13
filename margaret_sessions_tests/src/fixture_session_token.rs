use chrono::Utc;

use margaret_identity_session::session_access_token_claims::SessionAccessTokenClaims;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;

#[must_use]
pub fn fixture_session_token(store: &JwksSecretStore, claims: &SessionAccessTokenClaims) -> String {
    store
        .issue_session_access_token(claims, FIXTURE_AUDIENCE, Utc::now())
        .signed_claims
}

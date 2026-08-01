use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use margaret_identity_session::is_expired::IsExpired;
use margaret_jwt_claims::audience::Audience;
use margaret_jwt_claims::has_audience::HasAudience;
use margaret_jwt_claims::has_issuer::HasIssuer;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TestClaims {
    pub aud: Audience,
    pub exp: i64,
    pub iss: String,
    pub sub: String,
}

impl HasAudience for TestClaims {
    fn audience(&self) -> &Audience {
        &self.aud
    }
}

impl HasIssuer for TestClaims {
    fn issuer(&self) -> &str {
        &self.iss
    }
}

impl IsExpired for TestClaims {
    fn is_expired(&self, now: DateTime<Utc>) -> anyhow::Result<bool> {
        Ok(self.exp < now.timestamp())
    }
}

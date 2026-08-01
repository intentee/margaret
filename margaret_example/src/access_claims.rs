use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use margaret::framework::identity_session::is_expired::IsExpired;
use margaret::framework::jwt_claims::audience::Audience;
use margaret::framework::jwt_claims::has_audience::HasAudience;
use margaret::framework::jwt_claims::has_issuer::HasIssuer;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AccessClaims {
    pub aud: Audience,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub jti: Uuid,
    pub sub: String,
}

impl HasAudience for AccessClaims {
    fn audience(&self) -> &Audience {
        &self.aud
    }
}

impl HasIssuer for AccessClaims {
    fn issuer(&self) -> &str {
        &self.iss
    }
}

impl IsExpired for AccessClaims {
    fn is_expired(&self, now: DateTime<Utc>) -> anyhow::Result<bool> {
        Ok(self.exp < now.timestamp())
    }
}

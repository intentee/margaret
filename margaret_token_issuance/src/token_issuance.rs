use std::time::Duration;

use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret_jwt_verification::expected_audience::ExpectedAudience;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenIssuance {
    pub audience: Audience,
    pub issuer: IssuerIdentifier,
}

impl TokenIssuance {
    #[must_use]
    pub fn expectation(&self) -> JwtExpectation<'_> {
        JwtExpectation {
            audience: ExpectedAudience::One(&self.audience),
            issuer: &self.issuer,
        }
    }

    #[must_use]
    pub fn identified_claims(&self, now: DateTime<Utc>, lifetime_seconds: u32) -> RegisteredClaims {
        RegisteredClaims {
            jti: Some(Uuid::new_v4().to_string()),
            ..self.registered_claims(now, lifetime_seconds)
        }
    }

    #[must_use]
    pub fn registered_claims(&self, now: DateTime<Utc>, lifetime_seconds: u32) -> RegisteredClaims {
        let iat = NumericDate::from(now);

        RegisteredClaims {
            aud: AudienceClaim::Single(self.audience.as_str().to_string()),
            exp: iat.after(Duration::from_secs(u64::from(lifetime_seconds))),
            iat,
            iss: self.issuer.as_str().to_string(),
            jti: None,
            nbf: None,
        }
    }
}

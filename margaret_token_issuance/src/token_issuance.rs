use chrono::DateTime;
use chrono::Utc;

use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

use crate::declares_token_issuance::DeclaresTokenIssuance;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenIssuance {
    pub audience: Audience,
    pub issuer: IssuerIdentifier,
}

impl TokenIssuance {
    #[must_use]
    pub fn registered_claims(&self, now: DateTime<Utc>, lifetime_seconds: u32) -> RegisteredClaims {
        let iat = NumericDate::from(now);

        RegisteredClaims {
            aud: AudienceClaim::Single(self.audience.as_str().to_string()),
            exp: NumericDate::new(iat.seconds_since_epoch() + i64::from(lifetime_seconds)),
            iat,
            iss: self.issuer.as_str().to_string(),
            nbf: None,
        }
    }
}

impl DeclaresTokenIssuance for TokenIssuance {
    fn token_issuance(&self) -> &TokenIssuance {
        self
    }
}

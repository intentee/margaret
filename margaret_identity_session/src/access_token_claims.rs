use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::accepts_claims::AcceptsClaims;
use crate::claims_acceptance::ClaimsAcceptance;
use crate::claims_rejection::ClaimsRejection;

#[derive(Clone, Deserialize, Serialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl AcceptsClaims for AccessTokenClaims {
    fn accepts(&self, now: DateTime<Utc>) -> anyhow::Result<ClaimsAcceptance> {
        if self.exp < now.timestamp() {
            return Ok(ClaimsAcceptance::Rejected(ClaimsRejection::Expired));
        }

        Ok(ClaimsAcceptance::Accepted)
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use chrono::Utc;
    use uuid::Uuid;

    use super::AccessTokenClaims;
    use crate::accepts_claims::AcceptsClaims;
    use crate::claims_acceptance::ClaimsAcceptance;
    use crate::claims_rejection::ClaimsRejection;

    fn at(secs: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(secs, 0).expect("a valid timestamp")
    }

    fn claims(exp: i64) -> AccessTokenClaims {
        AccessTokenClaims {
            sub: Uuid::from_u128(1),
            exp,
            iat: 0,
        }
    }

    #[test]
    fn reports_expiry_relative_to_now() {
        assert_eq!(
            claims(101).accepts(at(100)).ok(),
            Some(ClaimsAcceptance::Accepted)
        );
        assert_eq!(
            claims(99).accepts(at(100)).ok(),
            Some(ClaimsAcceptance::Rejected(ClaimsRejection::Expired))
        );
        assert_eq!(
            claims(100).accepts(at(100)).ok(),
            Some(ClaimsAcceptance::Accepted)
        );
    }
}

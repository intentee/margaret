use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use margaret_identity_session::accepts_claims::AcceptsClaims;
use margaret_identity_session::claims_acceptance::ClaimsAcceptance;
use margaret_identity_session::claims_rejection::ClaimsRejection;

use crate::test_audience::TEST_AUDIENCE;
use crate::test_issuer::TEST_ISSUER;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TestClaims {
    pub aud: String,
    pub exp: i64,
    pub iss: String,
    pub nbf: i64,
    pub sub: String,
}

impl AcceptsClaims for TestClaims {
    fn accepts(&self, now: DateTime<Utc>) -> anyhow::Result<ClaimsAcceptance> {
        if self.iss != TEST_ISSUER {
            return Ok(ClaimsAcceptance::Rejected(
                ClaimsRejection::UnexpectedIssuer,
            ));
        }

        if self.aud != TEST_AUDIENCE {
            return Ok(ClaimsAcceptance::Rejected(
                ClaimsRejection::UnexpectedAudience,
            ));
        }

        if now.timestamp() < self.nbf {
            return Ok(ClaimsAcceptance::Rejected(ClaimsRejection::NotYetValid));
        }

        if self.exp < now.timestamp() {
            return Ok(ClaimsAcceptance::Rejected(ClaimsRejection::Expired));
        }

        Ok(ClaimsAcceptance::Accepted)
    }
}

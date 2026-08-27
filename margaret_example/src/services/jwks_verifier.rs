use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use tokio_util::sync::CancellationToken;

use margaret::framework::identity_session::accepts_claims::AcceptsClaims;
use margaret::framework::identity_session::claims_acceptance::ClaimsAcceptance;
use margaret::framework::identity_session::claims_rejection::ClaimsRejection;
use margaret::framework::jwks_client::access_token_verification::AccessTokenVerification;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::margaret::jwks::jwks_endpoint_jwks_endpoint::PublicJwksVerifier;
use crate::system_clock::SystemClock;

const ACCESS_TOKEN_AUDIENCE: &str = "margaret-example";
const ACCESS_TOKEN_ISSUER: &str = "https://margaret-example.invalid";

#[derive(Deserialize)]
struct AccessClaims {
    aud: String,
    exp: i64,
    iss: String,
}

impl AcceptsClaims for AccessClaims {
    fn accepts(&self, now: DateTime<Utc>) -> anyhow::Result<ClaimsAcceptance> {
        if self.iss != ACCESS_TOKEN_ISSUER {
            return Ok(ClaimsAcceptance::Rejected(
                ClaimsRejection::UnexpectedIssuer,
            ));
        }

        if self.aud != ACCESS_TOKEN_AUDIENCE {
            return Ok(ClaimsAcceptance::Rejected(
                ClaimsRejection::UnexpectedAudience,
            ));
        }

        if self.exp < now.timestamp() {
            return Ok(ClaimsAcceptance::Rejected(ClaimsRejection::Expired));
        }

        Ok(ClaimsAcceptance::Accepted)
    }
}

#[service]
pub struct JwksVerifier {
    clock: Arc<SystemClock>,
    verifier: Arc<PublicJwksVerifier>,
}

impl JwksVerifier {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        clock: Arc<SystemClock>,
        #[jwks_secret_store(client = auth)] verifier: Arc<PublicJwksVerifier>,
    ) -> anyhow::Result<Self> {
        Ok(Self { clock, verifier })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        match self
            .verifier
            .verify::<AccessClaims>("sample.access.token", self.clock.now())
        {
            Ok(AccessTokenVerification::Verified(_)) => {
                println!("the jwks verifier accepted the sample access token");
            }
            Ok(AccessTokenVerification::Rejected(rejection)) => {
                println!("the sample access token was rejected: {rejection:?}");
            }
            Ok(AccessTokenVerification::Malformed(malformation)) => {
                println!("the sample access token is malformed: {malformation}");
            }
            Ok(AccessTokenVerification::NotReady) => {
                println!("the jwks document has not been polled yet");
            }
            Ok(AccessTokenVerification::SignatureMismatch) => {
                println!("the sample access token signature does not match a published key");
            }
            Err(error) => println!("the jwks verifier could not verify the sample token: {error}"),
        }

        cancellation_token.cancelled().await;

        Ok(())
    }
}

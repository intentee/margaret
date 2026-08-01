use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret::framework::jwks_client::access_token_verification::AccessTokenVerification;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::access_claims::AccessClaims;
use crate::margaret::jwks::jwks_endpoint_jwks_endpoint::PublicJwksVerifier;
use crate::system_clock::SystemClock;

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
            Ok(AccessTokenVerification::AudienceMismatch) => {
                println!("the sample access token was minted for another audience");
            }
            Ok(AccessTokenVerification::Expired) => {
                println!("the sample access token is expired");
            }
            Ok(AccessTokenVerification::IssuerMismatch) => {
                println!("the sample access token came from another issuer");
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
            Ok(AccessTokenVerification::Verified(_)) => {
                println!("the jwks verifier accepted the sample access token");
            }
            Err(error) => println!("the jwks verifier could not verify the sample token: {error}"),
        }

        cancellation_token.cancelled().await;

        Ok(())
    }
}

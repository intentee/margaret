use std::sync::Arc;

use serde::Deserialize;
use tokio_util::sync::CancellationToken;

use margaret::framework::jwks_client::access_token_verification::AccessTokenVerification;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::margaret::jwks::jwks_endpoint_jwks_endpoint::PublicJwksVerifier;
use crate::system_clock::SystemClock;

#[derive(Deserialize)]
struct AccessClaims {}

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
            AccessTokenVerification::Verified(_) => {
                println!("the jwks verifier accepted the sample access token");
            }
            AccessTokenVerification::NotReady => {
                println!("the jwks document has not been polled yet");
            }
            AccessTokenVerification::Rejected(rejection) => {
                println!("the sample access token is rejected: {rejection}");
            }
        }

        cancellation_token.cancelled().await;

        Ok(())
    }
}

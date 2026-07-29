use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use tokio_util::sync::CancellationToken;

use margaret::framework::identity_session::is_expired::IsExpired;
use margaret::framework::jwks_client::access_token_verification::AccessTokenVerification;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::margaret::jwks::jwks_endpoint_jwks_endpoint::PublicJwksVerifier;
use crate::system_clock::SystemClock;

#[derive(Deserialize)]
struct AccessClaims {
    exp: i64,
}

impl IsExpired for AccessClaims {
    fn is_expired(&self, now: DateTime<Utc>) -> anyhow::Result<bool> {
        Ok(self.exp < now.timestamp())
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
            Ok(AccessTokenVerification::Expired) => {
                println!("the sample access token is expired");
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

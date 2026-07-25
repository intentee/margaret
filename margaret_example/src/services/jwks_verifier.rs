use std::convert::Infallible;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use tokio_util::sync::CancellationToken;

use margaret::framework::identity_session::is_expired::IsExpired;
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
    fn is_expired(&self, now: DateTime<Utc>) -> bool {
        self.exp < now.timestamp()
    }
}

#[service]
pub struct JwksVerifier {
    clock: Arc<SystemClock>,
    verifier: Arc<PublicJwksVerifier>,
}

impl JwksVerifier {
    #[constructor]
    #[must_use]
    pub fn create(
        clock: Arc<SystemClock>,
        #[jwks_secret_store(client = auth)] verifier: Arc<PublicJwksVerifier>,
    ) -> Self {
        Self { clock, verifier }
    }

    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> Result<(), Infallible> {
        match self
            .verifier
            .verify::<AccessClaims>("sample.access.token", self.clock.now())
        {
            Ok(_) => println!("the jwks verifier accepted the sample access token"),
            Err(error) => println!("the jwks verifier could not verify the sample token: {error}"),
        }

        cancellation_token.cancelled().await;

        Ok(())
    }
}

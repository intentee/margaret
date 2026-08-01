use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use margaret::framework::identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret::framework::jwt_claims::audience::Audience;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::access_claims::AccessClaims;
use crate::access_token_audience::ACCESS_TOKEN_AUDIENCE;
use crate::access_token_issuer::ACCESS_TOKEN_ISSUER;
use crate::margaret::jwks::JwksSecretStore;
use crate::system_clock::SystemClock;

#[service]
pub struct TokenIssuer {
    clock: Arc<SystemClock>,
    secret_store: Arc<JwksSecretStore>,
}

impl TokenIssuer {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        clock: Arc<SystemClock>,
        #[jwks_secret_store(server)] secret_store: Arc<JwksSecretStore>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            clock,
            secret_store,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        let issued_at = self.clock.now().timestamp();

        match self
            .secret_store
            .sign(&AccessClaims {
                aud: Audience::One(ACCESS_TOKEN_AUDIENCE.to_string()),
                exp: issued_at + ACCESS_TOKEN_LIFETIME_SECS,
                iat: issued_at,
                iss: ACCESS_TOKEN_ISSUER.to_string(),
                jti: Uuid::new_v4(),
                sub: "demo".to_string(),
            })
            .await
        {
            Ok(token) => println!("the token issuer signed a {} byte demo token", token.len()),
            Err(error) => println!("the token issuer could not sign a demo token: {error}"),
        }

        cancellation_token.cancelled().await;

        Ok(())
    }
}

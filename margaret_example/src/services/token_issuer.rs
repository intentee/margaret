use std::sync::Arc;

use serde::Serialize;
use tokio_util::sync::CancellationToken;

use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::margaret::jwks::JwksSecretStore;

#[derive(Serialize)]
struct DemoClaims {
    sub: String,
}

#[service]
pub struct TokenIssuer {
    secret_store: Arc<JwksSecretStore>,
}

impl TokenIssuer {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[jwks_secret_store(server)] secret_store: Arc<JwksSecretStore>,
    ) -> anyhow::Result<Self> {
        Ok(Self { secret_store })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        match self
            .secret_store
            .sign(&DemoClaims {
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

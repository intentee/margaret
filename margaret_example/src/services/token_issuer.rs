use std::convert::Infallible;
use std::sync::Arc;

use serde::Serialize;
use tokio_util::sync::CancellationToken;

use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::service;

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
    #[constructor]
    #[must_use]
    pub fn create(#[jwks_secret_store(server)] secret_store: Arc<JwksSecretStore>) -> Self {
        Self { secret_store }
    }

    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> Result<(), Infallible> {
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

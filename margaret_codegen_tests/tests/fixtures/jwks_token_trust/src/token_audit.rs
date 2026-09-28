use std::sync::Arc;

use chrono::DateTime;
use serde_json::Value;
use serde_json::json;
use tokio_util::sync::CancellationToken;

use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::margaret::asset_bag::asset;
use crate::margaret::jwks::JwksSecretStore;
use crate::margaret::jwks::partner_endpoint_partner_endpoint::PublicJwksVerifier;

#[service]
pub struct TokenAudit {
    partner_verifier: Arc<PublicJwksVerifier>,
    secret_store: Arc<JwksSecretStore>,
}

impl TokenAudit {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[jwks_secret_store(client = partner)] partner_verifier: Arc<PublicJwksVerifier>,
        #[jwks_secret_store(server)] secret_store: Arc<JwksSecretStore>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            partner_verifier,
            secret_store,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        let _asset = asset!("resources/ts/app.ts");
        let _signing = self
            .secret_store
            .sign_access_token(&json!({ "scope": "audit" }), DateTime::UNIX_EPOCH)?;
        let _verification = self
            .partner_verifier
            .verify::<Value>("audit.access.token", DateTime::UNIX_EPOCH);

        cancellation_token.cancelled().await;

        Ok(())
    }
}

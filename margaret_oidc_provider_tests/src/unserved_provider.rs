use std::sync::Arc;

use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::fixture_roller::fixture_roller;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::provider_issuance::provider_issuance;

pub struct UnservedProvider {
    pub issuance: TokenIssuance,
    pub roller: Arc<JwksRoller>,
    pub secret_store: Arc<JwksSecretStore>,
}

impl UnservedProvider {
    pub async fn create() -> Self {
        let issuance = provider_issuance();
        let roller = fixture_roller().await;

        Self {
            secret_store: Arc::new(JwksSecretStore::create(Arc::clone(&roller), issuance)),
            issuance,
            roller,
        }
    }
}

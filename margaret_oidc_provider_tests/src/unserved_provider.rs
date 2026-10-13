use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::fixture_secrets::fixture_secrets;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::provider_issuance::provider_issuance;

pub struct UnservedProvider {
    pub issuance: TokenIssuance,
    pub secret_store: Arc<JwksSecretStore>,
    pub secrets: Arc<JwksSecretHolder>,
}

impl UnservedProvider {
    #[must_use]
    pub fn create() -> Self {
        let issuance = provider_issuance();
        let secrets = fixture_secrets();

        Self {
            secret_store: Arc::new(JwksSecretStore::create(Arc::clone(&secrets), issuance)),
            issuance,
            secrets,
        }
    }
}

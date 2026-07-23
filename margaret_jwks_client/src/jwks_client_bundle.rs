use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use trzcina::Service;
use trzcina::ServiceBundle;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

use crate::jwks_client_bundle_params::JwksClientBundleParams;
use crate::public_jwks_holder::PublicJwksHolder;
use crate::public_jwks_poll_service::PublicJwksPollService;
use crate::public_jwks_verifier::PublicJwksVerifier;

pub struct JwksClientBundle {
    endpoint_provider: Arc<dyn ProvidesEndpoint>,
    http_client: Client,
    public_jwks_holder: PublicJwksHolder,
}

impl JwksClientBundle {
    #[must_use]
    pub fn new(
        JwksClientBundleParams {
            endpoint_provider,
            http_client,
        }: JwksClientBundleParams,
    ) -> Self {
        Self {
            endpoint_provider,
            http_client,
            public_jwks_holder: PublicJwksHolder::default(),
        }
    }

    #[must_use]
    pub fn verifier(&self) -> Arc<PublicJwksVerifier> {
        Arc::new(PublicJwksVerifier::new(self.public_jwks_holder.clone()))
    }
}

#[async_trait]
impl ServiceBundle for JwksClientBundle {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        Ok(vec![Box::new(PublicJwksPollService {
            endpoint_provider: self.endpoint_provider,
            http_client: self.http_client,
            public_jwks_holder: self.public_jwks_holder,
        })])
    }
}

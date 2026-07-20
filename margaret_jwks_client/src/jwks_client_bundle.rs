use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use trzcina::Service;
use trzcina::ServiceBundle;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

use crate::build_jwks_http_client::build_jwks_http_client;
use crate::jwk_public_set_holder::JwkPublicSetHolder;
use crate::jwk_public_set_poll_service::JwkPublicSetPollService;
use crate::jwk_public_set_verifier::JwkPublicSetVerifier;
use crate::jwks_client_bundle_params::JwksClientBundleParams;
use crate::jwks_client_error::JwksClientError;

pub struct JwksClientBundle {
    endpoint: Arc<dyn ProvidesEndpoint>,
    http_client: Client,
    jwk_public_set_holder: JwkPublicSetHolder,
}

impl JwksClientBundle {
    pub fn new(
        JwksClientBundleParams {
            client_config,
            endpoint,
        }: JwksClientBundleParams,
    ) -> Result<Self, JwksClientError> {
        build_jwks_http_client(Client::builder().use_preconfigured_tls(client_config)).map(
            |http_client| Self {
                endpoint,
                http_client,
                jwk_public_set_holder: JwkPublicSetHolder::default(),
            },
        )
    }

    #[must_use]
    pub fn verifier(&self) -> Arc<JwkPublicSetVerifier> {
        Arc::new(JwkPublicSetVerifier::new(
            self.jwk_public_set_holder.clone(),
        ))
    }
}

#[async_trait]
impl ServiceBundle for JwksClientBundle {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        Ok(vec![Box::new(JwkPublicSetPollService {
            endpoint: self.endpoint,
            http_client: self.http_client,
            jwk_public_set_holder: self.jwk_public_set_holder,
        })])
    }
}

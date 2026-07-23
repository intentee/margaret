use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use trzcina::Service;
use trzcina::ServiceBundle;
use url::Url;

use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;

use crate::build_jwks_http_client::build_jwks_http_client;
use crate::jwks_client_bundle_params::JwksClientBundleParams;
use crate::jwks_client_error::JwksClientError;
use crate::public_jwks_holder::PublicJwksHolder;
use crate::public_jwks_poll_service::PublicJwksPollService;
use crate::public_jwks_verifier::PublicJwksVerifier;

fn well_known_jwks_url(issuer_url: &Url) -> Result<Url, JwksClientError> {
    issuer_url
        .join(WELL_KNOWN_JWKS_PATH)
        .map_err(|source| JwksClientError::IssuerUrlNotABase {
            issuer_url: issuer_url.to_string(),
            source,
        })
}

pub struct JwksClientBundle {
    http_client: Client,
    public_jwks_holder: PublicJwksHolder,
    jwks_url: Url,
}

impl JwksClientBundle {
    pub fn new(
        JwksClientBundleParams {
            client_config,
            issuer_url,
        }: JwksClientBundleParams,
    ) -> Result<Self, JwksClientError> {
        build_jwks_http_client(Client::builder().use_preconfigured_tls(client_config)).and_then(
            |http_client| {
                Ok(Self {
                    http_client,
                    public_jwks_holder: PublicJwksHolder::default(),
                    jwks_url: well_known_jwks_url(&issuer_url)?,
                })
            },
        )
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
            http_client: self.http_client,
            public_jwks_holder: self.public_jwks_holder,
            jwks_url: self.jwks_url,
        })])
    }
}

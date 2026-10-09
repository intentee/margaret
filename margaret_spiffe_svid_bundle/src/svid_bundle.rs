use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use rustls::ClientConfig;
use rustls::ServerConfig;
use trzcina::Service;
use trzcina::ServiceBundle;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid::svid_service::SvidService;
use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid::svid_side_params::SvidSideParams;
use margaret_spiffe_svid_client::svid_client_readiness::SvidClientReadiness;
use margaret_spiffe_svid_client::svid_client_side::SvidClientSide;
use margaret_spiffe_svid_client::svid_error::SvidError;
use margaret_spiffe_svid_server::svid_server_side::SvidServerSide;

use crate::svid_bundle_error::SvidBundleError;

pub struct SvidBundle {
    client_side: SvidClientSide,
    server_side: SvidServerSide,
    service: SvidService,
}

impl SvidBundle {
    /// # Errors
    ///
    /// Returns `SvidBundleError` when the svid crypto provider supports none of the safe default
    /// tls versions for either side.
    pub fn new(
        SvidServiceBundleParams {
            spiffe_trust_domain,
            spire_agent_addr,
        }: SvidServiceBundleParams,
    ) -> Result<Self, SvidBundleError> {
        let crypto_provider = svid_crypto_provider();
        let service = SvidService::new(spire_agent_addr);
        SvidClientSide::new(SvidSideParams {
            crypto_provider: Arc::clone(&crypto_provider),
            root_cert_store_holder: service.root_cert_store_holder(),
            spiffe_trust_domain: spiffe_trust_domain.clone(),
            svid_certified_key_holder: service.svid_certified_key_holder(),
        })
        .map_err(SvidBundleError::ClientSide)
        .and_then(|client_side| {
            SvidServerSide::new(SvidSideParams {
                crypto_provider,
                root_cert_store_holder: service.root_cert_store_holder(),
                spiffe_trust_domain,
                svid_certified_key_holder: service.svid_certified_key_holder(),
            })
            .map_err(SvidBundleError::ServerSide)
            .map(|server_side| Self {
                client_side,
                server_side,
                service,
            })
        })
    }

    #[must_use]
    pub fn client_config(&self) -> ClientConfig {
        self.client_side.client_config()
    }

    #[must_use]
    pub fn client_readiness(&self) -> SvidClientReadiness {
        self.client_side.client_readiness()
    }

    /// # Errors
    ///
    /// Returns `SvidError` propagated from the work it performs.
    pub fn reqwest_client(&self) -> Result<Client, SvidError> {
        self.client_side.reqwest_client()
    }

    #[must_use]
    pub fn server_config(&self) -> ServerConfig {
        self.server_side.server_config()
    }
}

#[async_trait]
impl ServiceBundle for SvidBundle {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        let mut services = self.service.into_common_services();

        services.push(Box::new(self.server_side.into_verifier_service()));
        services.push(Box::new(self.client_side.into_verifier_service()));

        Ok(services)
    }
}

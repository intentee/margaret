use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use rustls::ClientConfig;
use rustls::ServerConfig;
use trzcina::Service;
use trzcina::ServiceBundle;

use margaret_spiffe_svid::svid_service::SvidService;
use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid::svid_side_params::SvidSideParams;
use margaret_spiffe_svid_client::svid_client_readiness::SvidClientReadiness;
use margaret_spiffe_svid_client::svid_client_side::SvidClientSide;
use margaret_spiffe_svid_client::svid_error::SvidError;
use margaret_spiffe_svid_server::svid_server_side::SvidServerSide;

pub struct SvidBundle {
    client_side: SvidClientSide,
    server_side: SvidServerSide,
    service: SvidService,
}

impl SvidBundle {
    #[must_use]
    pub fn new(
        SvidServiceBundleParams {
            spiffe_trust_domain,
            spire_agent_addr,
        }: SvidServiceBundleParams,
    ) -> Self {
        let service = SvidService::new(spire_agent_addr);
        let client_side = SvidClientSide::new(SvidSideParams {
            root_cert_store_holder: service.root_cert_store_holder(),
            spiffe_trust_domain: spiffe_trust_domain.clone(),
            svid_certified_key_holder: service.svid_certified_key_holder(),
        });
        let server_side = SvidServerSide::new(SvidSideParams {
            root_cert_store_holder: service.root_cert_store_holder(),
            spiffe_trust_domain,
            svid_certified_key_holder: service.svid_certified_key_holder(),
        });

        Self {
            client_side,
            server_side,
            service,
        }
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

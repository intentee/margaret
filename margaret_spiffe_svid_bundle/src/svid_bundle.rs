use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use rustls::ClientConfig;
use rustls::ServerConfig;
use trzcina::Service;
use trzcina::ServiceBundle;

use margaret_spiffe_svid::SvidService;
use margaret_spiffe_svid::SvidServiceBundleParams;
use margaret_spiffe_svid::SvidSideParams;
use margaret_spiffe_svid_client::SvidClientSide;
use margaret_spiffe_svid_client::svid_client_readiness::SvidClientReadiness;
use margaret_spiffe_svid_client::svid_error::SvidError;
use margaret_spiffe_svid_server::SvidServerSide;

pub struct SvidBundle {
    svid_client_side: SvidClientSide,
    svid_server_side: SvidServerSide,
    svid_service: SvidService,
}

impl SvidBundle {
    #[must_use]
    pub fn new(
        SvidServiceBundleParams {
            spiffe_trust_domain,
            spire_agent_addr,
        }: SvidServiceBundleParams,
    ) -> Self {
        let svid_service = SvidService::new(spire_agent_addr);
        let svid_client_side = SvidClientSide::new(SvidSideParams {
            root_cert_store_holder: svid_service.root_cert_store_holder(),
            spiffe_trust_domain: spiffe_trust_domain.clone(),
            svid_certified_key_holder: svid_service.svid_certified_key_holder(),
        });
        let svid_server_side = SvidServerSide::new(SvidSideParams {
            root_cert_store_holder: svid_service.root_cert_store_holder(),
            spiffe_trust_domain,
            svid_certified_key_holder: svid_service.svid_certified_key_holder(),
        });

        Self {
            svid_client_side,
            svid_server_side,
            svid_service,
        }
    }

    #[must_use]
    pub fn client_config(&self) -> ClientConfig {
        self.svid_client_side.client_config()
    }

    #[must_use]
    pub fn client_readiness(&self) -> SvidClientReadiness {
        self.svid_client_side.client_readiness()
    }

    pub fn reqwest_client(&self) -> Result<Client, SvidError> {
        self.svid_client_side.reqwest_client()
    }

    #[must_use]
    pub fn server_config(&self) -> ServerConfig {
        self.svid_server_side.server_config()
    }
}

#[async_trait]
impl ServiceBundle for SvidBundle {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        let mut services = self.svid_service.into_common_services();

        services.push(Box::new(self.svid_server_side.into_verifier_service()));
        services.push(Box::new(self.svid_client_side.into_verifier_service()));

        Ok(services)
    }
}

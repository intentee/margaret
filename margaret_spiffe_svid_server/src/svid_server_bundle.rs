use anyhow::Result;
use async_trait::async_trait;
use rustls::ServerConfig;
use trzcina::Service;
use trzcina::ServiceBundle;

use margaret_spiffe_svid::svid_service::SvidService;
use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid::svid_side_params::SvidSideParams;

use crate::svid_server_side::SvidServerSide;

pub struct SvidServerBundle {
    svid_server_side: SvidServerSide,
    svid_service: SvidService,
}

impl SvidServerBundle {
    #[must_use]
    pub fn new(
        SvidServiceBundleParams {
            spiffe_trust_domain,
            spire_agent_addr,
        }: SvidServiceBundleParams,
    ) -> Self {
        let svid_service = SvidService::new(spire_agent_addr);
        let svid_server_side = SvidServerSide::new(SvidSideParams {
            root_cert_store_holder: svid_service.root_cert_store_holder(),
            spiffe_trust_domain,
            svid_certified_key_holder: svid_service.svid_certified_key_holder(),
        });

        Self {
            svid_server_side,
            svid_service,
        }
    }

    #[must_use]
    pub fn server_config(&self) -> ServerConfig {
        self.svid_server_side.server_config()
    }
}

#[async_trait]
impl ServiceBundle for SvidServerBundle {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        let mut services = self.svid_service.into_common_services();

        services.push(Box::new(self.svid_server_side.into_verifier_service()));

        Ok(services)
    }
}

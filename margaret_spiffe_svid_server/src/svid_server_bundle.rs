use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use rustls::ServerConfig;
use trzcina::Service;
use trzcina::ServiceBundle;

use margaret_spiffe_svid::SvidService;
use margaret_spiffe_svid::SvidServiceBundleParams;

use crate::svid_client_cert_verifier_facade::SvidClientCertVerifierFacade;
use crate::svid_client_cert_verifier_service::SvidClientCertVerifierService;

pub struct SvidServerBundle {
    server_config: ServerConfig,
    spiffe_trust_domain: String,
    svid_client_cert_verifier_facade: Arc<SvidClientCertVerifierFacade>,
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
        let svid_client_cert_verifier_facade = Arc::new(SvidClientCertVerifierFacade::default());

        let server_config: ServerConfig = ServerConfig::builder()
            .with_client_cert_verifier(svid_client_cert_verifier_facade.clone())
            .with_cert_resolver(Arc::new(svid_service.svid_certified_key_holder()));

        Self {
            server_config,
            spiffe_trust_domain,
            svid_client_cert_verifier_facade,
            svid_service,
        }
    }

    #[must_use]
    pub fn server_config(&self) -> ServerConfig {
        self.server_config.clone()
    }
}

#[async_trait]
impl ServiceBundle for SvidServerBundle {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        let root_cert_store_holder = self.svid_service.root_cert_store_holder();
        let mut services = self.svid_service.into_common_services();

        services.push(Box::new(SvidClientCertVerifierService {
            root_cert_store_holder,
            spiffe_trust_domain: self.spiffe_trust_domain,
            svid_client_cert_verifier_facade: self.svid_client_cert_verifier_facade,
        }));

        Ok(services)
    }
}

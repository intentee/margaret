use std::sync::Arc;

use rustls::ServerConfig;
use trzcina::Service;

use margaret_spiffe_svid::SvidService;

use crate::svid_client_cert_verifier_facade::SvidClientCertVerifierFacade;
use crate::svid_client_cert_verifier_service::SvidClientCertVerifierService;

pub struct SvidServer {
    server_config: ServerConfig,
    spiffe_trust_domain: String,
    svid_client_cert_verifier_facade: Arc<SvidClientCertVerifierFacade>,
    svid_service: SvidService,
}

impl SvidServer {
    #[must_use]
    pub fn new(spiffe_trust_domain: String, spire_agent_addr: String) -> Self {
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
    pub fn into_services(self) -> Vec<Box<dyn Service>> {
        let root_cert_store_holder = self.svid_service.root_cert_store_holder();
        let mut services = self.svid_service.into_common_services();

        services.push(Box::new(SvidClientCertVerifierService {
            root_cert_store_holder,
            spiffe_trust_domain: self.spiffe_trust_domain,
            svid_client_cert_verifier_facade: self.svid_client_cert_verifier_facade,
        }));

        services
    }

    #[must_use]
    pub fn server_config(&self) -> ServerConfig {
        self.server_config.clone()
    }
}

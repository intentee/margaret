use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use reqwest::tls::Version;
use rustls::ClientConfig;
use trzcina::Service;
use trzcina::ServiceBundle;

use margaret_spiffe_svid::SvidServiceBundleParams;
use margaret_spiffe_svid::SvidServiceCore;

use crate::build_reqwest_client::build_reqwest_client;
use crate::svid_error::SvidError;
use crate::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;
use crate::svid_server_cert_verifier_service::SvidServerCertVerifierService;

pub struct SvidClientBundle {
    client_config: ClientConfig,
    core: SvidServiceCore,
    spiffe_trust_domain: String,
    svid_server_cert_verifier_facade: Arc<SvidServerCertVerifierFacade>,
}

impl SvidClientBundle {
    #[must_use]
    pub fn new(
        SvidServiceBundleParams {
            spiffe_trust_domain,
            spire_agent_addr,
        }: SvidServiceBundleParams,
    ) -> Self {
        let core = SvidServiceCore::new(spire_agent_addr);
        let svid_server_cert_verifier_facade = Arc::new(SvidServerCertVerifierFacade::default());

        let client_config: ClientConfig = ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(svid_server_cert_verifier_facade.clone())
            .with_client_cert_resolver(Arc::new(core.svid_certified_key_holder()));

        Self {
            client_config,
            core,
            spiffe_trust_domain,
            svid_server_cert_verifier_facade,
        }
    }

    #[must_use]
    pub fn client_config(&self) -> ClientConfig {
        self.client_config.clone()
    }

    pub fn reqwest_client(&self) -> Result<Client, SvidError> {
        build_reqwest_client(
            Client::builder()
                .use_preconfigured_tls(self.client_config())
                .min_tls_version(Version::TLS_1_3),
        )
    }
}

#[async_trait]
impl ServiceBundle for SvidClientBundle {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        let root_cert_store_holder = self.core.root_cert_store_holder();
        let mut services = self.core.into_common_services();

        services.push(Box::new(SvidServerCertVerifierService {
            root_cert_store_holder,
            spiffe_trust_domain: self.spiffe_trust_domain,
            svid_server_cert_verifier_facade: self.svid_server_cert_verifier_facade,
        }));

        Ok(services)
    }
}

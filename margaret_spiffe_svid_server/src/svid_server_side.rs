use std::sync::Arc;

use rustls::ServerConfig;

use margaret_spiffe_svid::SvidSideParams;
use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;

use crate::svid_client_cert_verifier_facade::SvidClientCertVerifierFacade;
use crate::svid_client_cert_verifier_service::SvidClientCertVerifierService;

pub struct SvidServerSide {
    root_cert_store_holder: RootCertStoreHolder,
    server_config: ServerConfig,
    spiffe_trust_domain: String,
    svid_client_cert_verifier_facade: Arc<SvidClientCertVerifierFacade>,
}

impl SvidServerSide {
    #[must_use]
    pub fn new(
        SvidSideParams {
            root_cert_store_holder,
            spiffe_trust_domain,
            svid_certified_key_holder,
        }: SvidSideParams,
    ) -> Self {
        let svid_client_cert_verifier_facade = Arc::new(SvidClientCertVerifierFacade::default());

        let server_config: ServerConfig =
            ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
                .with_client_cert_verifier(svid_client_cert_verifier_facade.clone())
                .with_cert_resolver(Arc::new(svid_certified_key_holder));

        Self {
            root_cert_store_holder,
            server_config,
            spiffe_trust_domain,
            svid_client_cert_verifier_facade,
        }
    }

    #[must_use]
    pub fn into_verifier_service(self) -> SvidClientCertVerifierService {
        SvidClientCertVerifierService {
            root_cert_store_holder: self.root_cert_store_holder,
            spiffe_trust_domain: self.spiffe_trust_domain,
            svid_client_cert_verifier_facade: self.svid_client_cert_verifier_facade,
        }
    }

    #[must_use]
    pub fn server_config(&self) -> ServerConfig {
        self.server_config.clone()
    }
}

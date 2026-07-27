use std::sync::Arc;

use reqwest::Client;
use reqwest::tls::Version;
use rustls::ClientConfig;

use margaret_spiffe_svid::SvidSideParams;
use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;

use crate::build_reqwest_client::build_reqwest_client;
use crate::svid_client_readiness::SvidClientReadiness;
use crate::svid_error::SvidError;
use crate::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;
use crate::svid_server_cert_verifier_service::SvidServerCertVerifierService;

pub struct SvidClientSide {
    client_config: ClientConfig,
    root_cert_store_holder: RootCertStoreHolder,
    spiffe_trust_domain: String,
    svid_server_cert_verifier_facade: Arc<SvidServerCertVerifierFacade>,
}

impl SvidClientSide {
    #[must_use]
    pub fn new(
        SvidSideParams {
            root_cert_store_holder,
            spiffe_trust_domain,
            svid_certified_key_holder,
        }: SvidSideParams,
    ) -> Self {
        let svid_server_cert_verifier_facade = Arc::new(SvidServerCertVerifierFacade::default());

        let client_config: ClientConfig =
            ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
                .dangerous()
                .with_custom_certificate_verifier(svid_server_cert_verifier_facade.clone())
                .with_client_cert_resolver(Arc::new(svid_certified_key_holder));

        Self {
            client_config,
            root_cert_store_holder,
            spiffe_trust_domain,
            svid_server_cert_verifier_facade,
        }
    }

    #[must_use]
    pub fn client_config(&self) -> ClientConfig {
        self.client_config.clone()
    }

    #[must_use]
    pub fn client_readiness(&self) -> SvidClientReadiness {
        SvidClientReadiness::new(self.svid_server_cert_verifier_facade.subscribe())
    }

    #[must_use]
    pub fn into_verifier_service(self) -> SvidServerCertVerifierService {
        SvidServerCertVerifierService {
            root_cert_store_holder: self.root_cert_store_holder,
            spiffe_trust_domain: self.spiffe_trust_domain,
            svid_server_cert_verifier_facade: self.svid_server_cert_verifier_facade,
        }
    }

    pub fn reqwest_client(&self) -> Result<Client, SvidError> {
        build_reqwest_client(
            Client::builder()
                .use_preconfigured_tls(self.client_config())
                .min_tls_version(Version::TLS_1_3),
        )
    }
}

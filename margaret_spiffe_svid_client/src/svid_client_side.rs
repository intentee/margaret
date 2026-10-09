use std::sync::Arc;

use reqwest::Client;
use reqwest::tls::Version;
use rustls::ClientConfig;
use rustls::crypto::CryptoProvider;

use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid::svid_side_params::SvidSideParams;

use crate::build_reqwest_client::build_reqwest_client;
use crate::svid_client_readiness::SvidClientReadiness;
use crate::svid_error::SvidError;
use crate::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;
use crate::svid_server_cert_verifier_service::SvidServerCertVerifierService;

pub struct SvidClientSide {
    client_config: ClientConfig,
    crypto_provider: Arc<CryptoProvider>,
    root_cert_store_holder: RootCertStoreHolder,
    spiffe_trust_domain: String,
    svid_server_cert_verifier_facade: Arc<SvidServerCertVerifierFacade>,
}

impl SvidClientSide {
    /// # Errors
    ///
    /// Returns `SvidError::ProtocolVersions` when the crypto provider supports none of the safe
    /// default tls versions.
    pub fn new(
        SvidSideParams {
            crypto_provider,
            root_cert_store_holder,
            spiffe_trust_domain,
            svid_certified_key_holder,
        }: SvidSideParams,
    ) -> Result<Self, SvidError> {
        let svid_server_cert_verifier_facade = Arc::new(SvidServerCertVerifierFacade::default());

        let client_config: ClientConfig =
            ClientConfig::builder_with_provider(Arc::clone(&crypto_provider))
                .with_safe_default_protocol_versions()
                .map_err(|source| SvidError::ProtocolVersions { source })?
                .dangerous()
                .with_custom_certificate_verifier(svid_server_cert_verifier_facade.clone())
                .with_client_cert_resolver(Arc::new(svid_certified_key_holder));

        Ok(Self {
            client_config,
            crypto_provider,
            root_cert_store_holder,
            spiffe_trust_domain,
            svid_server_cert_verifier_facade,
        })
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
            crypto_provider: self.crypto_provider,
            root_cert_store_holder: self.root_cert_store_holder,
            spiffe_trust_domain: self.spiffe_trust_domain,
            svid_server_cert_verifier_facade: self.svid_server_cert_verifier_facade,
        }
    }

    /// # Errors
    ///
    /// Returns `SvidError` propagated from the work it performs.
    pub fn reqwest_client(&self) -> Result<Client, SvidError> {
        build_reqwest_client(
            Client::builder()
                .use_preconfigured_tls(self.client_config())
                .min_tls_version(Version::TLS_1_3),
        )
    }
}

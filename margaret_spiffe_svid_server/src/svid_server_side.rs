use std::sync::Arc;

use rustls::ServerConfig;
use rustls::crypto::CryptoProvider;

use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;
use margaret_spiffe_svid::svid_side_params::SvidSideParams;

use crate::svid_client_cert_verifier_facade::SvidClientCertVerifierFacade;
use crate::svid_client_cert_verifier_service::SvidClientCertVerifierService;
use crate::svid_error::SvidError;

pub struct SvidServerSide {
    crypto_provider: Arc<CryptoProvider>,
    root_cert_store_holder: RootCertStoreHolder,
    server_config: ServerConfig,
    spiffe_trust_domain: String,
    svid_client_cert_verifier_facade: Arc<SvidClientCertVerifierFacade>,
}

impl SvidServerSide {
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
        let svid_client_cert_verifier_facade = Arc::new(SvidClientCertVerifierFacade::default());

        let server_config: ServerConfig =
            ServerConfig::builder_with_provider(Arc::clone(&crypto_provider))
                .with_safe_default_protocol_versions()
                .map_err(|source| SvidError::ProtocolVersions { source })?
                .with_client_cert_verifier(svid_client_cert_verifier_facade.clone())
                .with_cert_resolver(Arc::new(svid_certified_key_holder));

        Ok(Self {
            crypto_provider,
            root_cert_store_holder,
            server_config,
            spiffe_trust_domain,
            svid_client_cert_verifier_facade,
        })
    }

    #[must_use]
    pub fn into_verifier_service(self) -> SvidClientCertVerifierService {
        SvidClientCertVerifierService {
            crypto_provider: self.crypto_provider,
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

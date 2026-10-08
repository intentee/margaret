use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use rustls::ClientConfig;
use trzcina::Service;
use trzcina::ServiceBundle;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid::svid_service::SvidService;
use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;
use margaret_spiffe_svid::svid_side_params::SvidSideParams;

use crate::svid_client_readiness::SvidClientReadiness;
use crate::svid_client_side::SvidClientSide;
use crate::svid_error::SvidError;

pub struct SvidClientBundle {
    svid_client_side: SvidClientSide,
    svid_service: SvidService,
}

impl SvidClientBundle {
    /// # Errors
    ///
    /// Returns `SvidError::ProtocolVersions` when the svid crypto provider supports none of the
    /// safe default tls versions.
    pub fn new(
        SvidServiceBundleParams {
            spiffe_trust_domain,
            spire_agent_addr,
        }: SvidServiceBundleParams,
    ) -> Result<Self, SvidError> {
        let svid_service = SvidService::new(spire_agent_addr);

        SvidClientSide::new(SvidSideParams {
            crypto_provider: svid_crypto_provider(),
            root_cert_store_holder: svid_service.root_cert_store_holder(),
            spiffe_trust_domain,
            svid_certified_key_holder: svid_service.svid_certified_key_holder(),
        })
        .map(|svid_client_side| Self {
            svid_client_side,
            svid_service,
        })
    }

    #[must_use]
    pub fn client_config(&self) -> ClientConfig {
        self.svid_client_side.client_config()
    }

    #[must_use]
    pub fn client_readiness(&self) -> SvidClientReadiness {
        self.svid_client_side.client_readiness()
    }

    /// # Errors
    ///
    /// Returns `SvidError` propagated from the work it performs.
    pub fn reqwest_client(&self) -> Result<Client, SvidError> {
        self.svid_client_side.reqwest_client()
    }
}

#[async_trait]
impl ServiceBundle for SvidClientBundle {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        let mut services = self.svid_service.into_common_services();

        services.push(Box::new(self.svid_client_side.into_verifier_service()));

        Ok(services)
    }
}

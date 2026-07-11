use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use rustls::ServerConfig;
use tokio::sync::broadcast;
use trzcina::Service;
use trzcina::ServiceBundle;

use crate::reqwest_client_holder::ReqwestClientHolder;
use crate::reqwest_client_service::ReqwestClientService;
use crate::root_cert_store_holder::RootCertStoreHolder;
use crate::root_cert_store_service::RootCertStoreService;
use crate::svid_certified_key_holder::SvidCertifiedKeyHolder;
use crate::svid_client_cert_verifier::SvidClientCertVerifier;
use crate::svid_client_cert_verifier_service::SvidClientCertVerifierService;
use crate::svid_converter_service::SvidConverterService;
use crate::svid_rotate_service::SvidRotateService;
use crate::svid_service_bundle_params::SvidServiceBundleParams;

pub struct SvidServiceBundle {
    reqwest_client_holder: ReqwestClientHolder,
    root_cert_store_holder: RootCertStoreHolder,
    server_config: ServerConfig,
    spiffe_trust_domain: String,
    spire_agent_addr: String,
    svid_certified_key_holder: SvidCertifiedKeyHolder,
    svid_client_cert_verifier: Arc<SvidClientCertVerifier>,
}

impl SvidServiceBundle {
    #[must_use]
    pub fn new(
        SvidServiceBundleParams {
            spiffe_trust_domain,
            spire_agent_addr,
        }: SvidServiceBundleParams,
    ) -> Self {
        let svid_certified_key_holder: SvidCertifiedKeyHolder = Default::default();
        let svid_client_cert_verifier = Arc::new(SvidClientCertVerifier::default());

        let server_config: ServerConfig = ServerConfig::builder()
            .with_client_cert_verifier(svid_client_cert_verifier.clone())
            .with_cert_resolver(Arc::new(svid_certified_key_holder.clone()));

        Self {
            reqwest_client_holder: Default::default(),
            root_cert_store_holder: Default::default(),
            server_config,
            spiffe_trust_domain,
            spire_agent_addr,
            svid_certified_key_holder,
            svid_client_cert_verifier,
        }
    }

    #[must_use]
    pub fn reqwest_client_holder(&self) -> ReqwestClientHolder {
        self.reqwest_client_holder.clone()
    }

    #[must_use]
    pub fn server_config(&self) -> ServerConfig {
        self.server_config.clone()
    }
}

#[async_trait]
impl ServiceBundle for SvidServiceBundle {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        let (ca_bundle_tx, ca_bundle_rx) = broadcast::channel(1);
        let (x509_context_tx, x509_context_rx) = broadcast::channel(1);

        Ok(vec![
            Box::new(SvidRotateService {
                spire_agent_addr: self.spire_agent_addr,
                x509_context_tx,
            }),
            Box::new(SvidConverterService {
                ca_bundle_tx,
                svid_certified_key_holder: self.svid_certified_key_holder.clone(),
                x509_context_rx,
            }),
            Box::new(RootCertStoreService {
                ca_bundle_rx,
                root_cert_store_holder: self.root_cert_store_holder.clone(),
            }),
            Box::new(SvidClientCertVerifierService {
                svid_client_cert_verifier: self.svid_client_cert_verifier.clone(),
                root_cert_store_holder: self.root_cert_store_holder.clone(),
            }),
            Box::new(ReqwestClientService {
                reqwest_client_holder: self.reqwest_client_holder,
                root_cert_store_holder: self.root_cert_store_holder,
                spiffe_trust_domain: self.spiffe_trust_domain,
                svid_certified_key_holder: self.svid_certified_key_holder,
            }),
        ])
    }
}

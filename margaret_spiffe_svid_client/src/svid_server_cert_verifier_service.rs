use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use rustls::RootCertStore;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_spiffe_svid::root_cert_store_holder::RootCertStoreHolder;

use crate::svid_server_cert_verifier::SvidServerCertVerifier;
use crate::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;

pub struct SvidServerCertVerifierService {
    pub root_cert_store_holder: RootCertStoreHolder,
    pub spiffe_trust_domain: String,
    pub svid_server_cert_verifier_facade: Arc<SvidServerCertVerifierFacade>,
}

impl SvidServerCertVerifierService {
    fn update_server_cert_verifier(&self, root_cert_store: Option<RootCertStore>) -> Result<()> {
        if let Some(root_store) = root_cert_store {
            let verifier =
                SvidServerCertVerifier::new(root_store, &self.spiffe_trust_domain)?;

            self.svid_server_cert_verifier_facade
                .update_internal_verifier(Arc::new(verifier));
        }

        Ok(())
    }
}

#[async_trait]
impl Service for SvidServerCertVerifierService {
    async fn run(self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        let mut root_cert_store_subscription = self.root_cert_store_holder.subscribe();

        loop {
            let root_cert_store = root_cert_store_subscription.read_current();

            self.update_server_cert_verifier(root_cert_store)?;

            tokio::select! {
                () = cancellation_token.cancelled() => return Ok(()),
                () = root_cert_store_subscription.changed() => {},
            }
        }
    }
}

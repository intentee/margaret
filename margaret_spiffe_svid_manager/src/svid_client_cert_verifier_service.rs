use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use rustls::RootCertStore;
use rustls::server::WebPkiClientVerifier;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use crate::root_cert_store_holder::RootCertStoreHolder;
use crate::svid_client_cert_verifier::SvidClientCertVerifier;
use crate::svid_error::SvidError;

pub struct SvidClientCertVerifierService {
    pub root_cert_store_holder: RootCertStoreHolder,
    pub svid_client_cert_verifier: Arc<SvidClientCertVerifier>,
}

impl SvidClientCertVerifierService {
    async fn update_client_cert_verifier(
        &self,
        root_cert_store: Option<RootCertStore>,
    ) -> Result<()> {
        if let Some(root_store) = root_cert_store {
            let verifier = WebPkiClientVerifier::builder(Arc::new(root_store))
                .build()
                .map_err(|source| SvidError::ClientVerifier { source })?;

            self.svid_client_cert_verifier
                .update_internal_verifier(verifier);
        }

        Ok(())
    }
}

#[async_trait]
impl Service for SvidClientCertVerifierService {
    async fn run(self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        let mut root_cert_store_subscription = self.root_cert_store_holder.subscribe();

        loop {
            let root_cert_store = root_cert_store_subscription.read_current();

            self.update_client_cert_verifier(root_cert_store).await?;

            tokio::select! {
                () = cancellation_token.cancelled() => return Ok(()),
                () = root_cert_store_subscription.changed() => {},
            }
        }
    }
}

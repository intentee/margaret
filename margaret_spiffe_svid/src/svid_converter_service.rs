use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use log::error;
use log::info;
use log::warn;
use spiffe::X509Context;
use spiffe::X509Svid;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::error::RecvError;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use crate::ca_bundle::CaBundle;
use crate::extract_ca_bundle::extract_ca_bundle;
use crate::extract_server_credentials::extract_server_credentials;
use crate::svid_certified_key_holder::SvidCertifiedKeyHolder;
use crate::svid_error::SvidError;

pub struct SvidConverterService {
    pub ca_bundle_tx: Sender<Arc<CaBundle<'static>>>,
    pub svid_certified_key_holder: SvidCertifiedKeyHolder,
    pub x509_context_rx: Receiver<X509Context>,
}

impl SvidConverterService {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn convert_x509_context(&self, x509_context: &X509Context) -> Result<()> {
        let default_svid: &X509Svid = x509_context
            .default_svid()
            .ok_or(SvidError::MissingDefaultSvid)?;

        self.svid_certified_key_holder
            .set(Some(Arc::new(extract_server_credentials(default_svid)?)));
        self.ca_bundle_tx
            .send(extract_ca_bundle(default_svid, x509_context)?.into())?;

        Ok(())
    }
}

#[async_trait]
impl Service for SvidConverterService {
    async fn run(mut self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        loop {
            tokio::select! {
                biased;
                () = cancellation_token.cancelled() => return Ok(()),
                x509_context_result = self.x509_context_rx.recv() => {
                    match x509_context_result {
                        Ok(x509_context) => {
                            info!("Received x509 SVID context");

                            if let Err(err) = self.convert_x509_context(&x509_context) {
                                error!("Unable to process SVID context: {err:#?}");
                            }
                        }
                        Err(RecvError::Lagged(skipped)) => {
                            warn!("Lagged while receiving SVID context, skipped {skipped} messages");
                        }
                        Err(RecvError::Closed) => {
                            return Err(anyhow!("Unable to receive a new SVID: channel closed"));
                        }
                    }
                },
            }
        }
    }
}

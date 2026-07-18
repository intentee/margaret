use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use log::warn;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::error::RecvError;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use crate::ca_bundle::CaBundle;
use crate::root_cert_store_holder::RootCertStoreHolder;

pub struct RootCertStoreService {
    pub ca_bundle_rx: Receiver<Arc<CaBundle<'static>>>,
    pub root_cert_store_holder: RootCertStoreHolder,
}

#[async_trait]
impl Service for RootCertStoreService {
    async fn run(mut self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        loop {
            tokio::select! {
                biased;
                () = cancellation_token.cancelled() => return Ok(()),
                ca_bundle_result = self.ca_bundle_rx.recv() => {
                    match ca_bundle_result {
                        Ok(ca_bundle) => {
                            self.root_cert_store_holder.set(
                                Some(ca_bundle.to_root_cert_store()?)
                            );
                        }
                        Err(RecvError::Lagged(skipped)) => {
                            warn!("Lagged while receiving SVID CA bundle, skipped {skipped} messages");
                        }
                        Err(RecvError::Closed) => {
                            return Err(anyhow!("Unable to receive SVID CA bundle: channel closed"));
                        }
                    }
                }
            }
        }
    }
}

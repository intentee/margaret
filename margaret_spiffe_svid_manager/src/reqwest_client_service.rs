use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use reqwest::ClientBuilder;
use reqwest::tls::Version;
use rustls::RootCertStore;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use crate::build_rustls_client_config::build_rustls_client_config;
use crate::reqwest_client_holder::ReqwestClientHolder;
use crate::root_cert_store_holder::RootCertStoreHolder;
use crate::svid_certified_key::SvidCertifiedKey;
use crate::svid_certified_key_holder::SvidCertifiedKeyHolder;

fn build_reqwest_client(client_builder: ClientBuilder) -> Result<Client> {
    Ok(client_builder.build()?)
}

pub struct ReqwestClientService {
    pub reqwest_client_holder: ReqwestClientHolder,
    pub root_cert_store_holder: RootCertStoreHolder,
    pub spiffe_trust_domain: String,
    pub svid_certified_key_holder: SvidCertifiedKeyHolder,
}

impl ReqwestClientService {
    fn update_rustls_client(
        &self,
        root_cert_store: Option<RootCertStore>,
        svid_certified_key: Option<Arc<SvidCertifiedKey>>,
    ) -> Result<()> {
        match (root_cert_store, svid_certified_key) {
            (Some(root_store), Some(svid_certified_key)) => {
                let client_config = build_rustls_client_config(
                    root_store,
                    svid_certified_key,
                    self.spiffe_trust_domain.clone(),
                )?;

                build_reqwest_client(
                    Client::builder()
                        .use_preconfigured_tls(client_config)
                        .min_tls_version(Version::TLS_1_3),
                )
                .map(|client| self.reqwest_client_holder.set(Some(client)))
            }
            _ => {
                self.reqwest_client_holder.set(None);

                Ok(())
            }
        }
    }
}

#[async_trait]
impl Service for ReqwestClientService {
    async fn run(self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        let mut root_cert_store_subscription = self.root_cert_store_holder.subscribe();
        let mut svid_certified_key_subscription = self.svid_certified_key_holder.subscribe();

        loop {
            let root_cert_store = root_cert_store_subscription.read_current();
            let svid_certified_key = svid_certified_key_subscription.read_current();

            self.update_rustls_client(root_cert_store, svid_certified_key)?;

            tokio::select! {
                () = cancellation_token.cancelled() => return Ok(()),
                () = root_cert_store_subscription.changed() => {},
                () = svid_certified_key_subscription.changed() => {},
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::build_reqwest_client;

    #[test]
    fn build_reqwest_client_surfaces_builder_errors() {
        let builder_with_invalid_tls_backend =
            reqwest::Client::builder().use_preconfigured_tls(0u8);

        assert!(build_reqwest_client(builder_with_invalid_tls_backend).is_err());
    }
}

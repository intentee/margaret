use std::sync::Arc;

use margaret_issuer_directory::discovered_issuer::DiscoveredIssuer;
use margaret_issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer;
use margaret_issuer_directory::polled_key_set::PolledKeySet;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_token_trust::token_trust::TokenTrust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::polled_directory::PolledDirectory;

pub struct PolledFixture {
    pub key_set: Arc<IssuerKeySet>,
    pub polled: Arc<PolledKeySet>,
}

impl PolledFixture {
    #[must_use]
    pub fn discovered(issuer: DiscoveredIssuer, metadata: Arc<IssuerMetadata>) -> Self {
        let key_set = Arc::new(IssuerKeySet::awaiting());

        Self {
            polled: Arc::new(PolledKeySet::discovered(
                issuer,
                metadata,
                Arc::clone(&key_set),
            )),
            key_set,
        }
    }

    #[must_use]
    pub fn published(issuer: JwksEndpointIssuer) -> Self {
        let key_set = Arc::new(IssuerKeySet::awaiting());

        Self {
            polled: Arc::new(PolledKeySet::published(issuer, Arc::clone(&key_set))),
            key_set,
        }
    }

    pub async fn first_poll(&self, request_client: IssuerRequestClient) -> KeySetRefresh {
        let snapshot = self.key_set.snapshot();
        let directory = PolledDirectory::start(vec![Arc::clone(&self.polled)], request_client);
        let refresh = self.key_set.refreshed_since(&snapshot).await;

        directory.stop().await;

        refresh
    }

    #[must_use]
    pub fn trusted(&self, trust: TokenTrust) -> TrustedIssuer {
        TrustedIssuer::create(Arc::clone(&self.key_set), trust)
    }
}

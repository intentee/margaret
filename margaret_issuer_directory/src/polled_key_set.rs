use std::sync::Arc;

use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;

use crate::discovered_issuer::DiscoveredIssuer;
use crate::jwks_endpoint_issuer::JwksEndpointIssuer;
use crate::key_set_source::KeySetSource;

pub struct PolledKeySet {
    pub(crate) key_set: Arc<IssuerKeySet>,
    pub(crate) source: KeySetSource,
}

impl PolledKeySet {
    #[must_use]
    pub fn discovered(
        issuer: DiscoveredIssuer,
        metadata: Arc<IssuerMetadata>,
        key_set: Arc<IssuerKeySet>,
    ) -> Self {
        Self {
            key_set,
            source: KeySetSource::Discovered { issuer, metadata },
        }
    }

    #[must_use]
    pub fn published(issuer: JwksEndpointIssuer, key_set: Arc<IssuerKeySet>) -> Self {
        Self {
            key_set,
            source: KeySetSource::Published(issuer),
        }
    }
}

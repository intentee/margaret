use std::sync::Arc;

use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;

use crate::discovered_issuer::DiscoveredIssuer;
use crate::jwks_endpoint_issuer::JwksEndpointIssuer;

pub(crate) enum KeySetSource {
    Discovered {
        issuer: DiscoveredIssuer,
        metadata: Arc<IssuerMetadata>,
    },
    Published(JwksEndpointIssuer),
}

impl KeySetSource {
    pub(crate) fn issuer(&self) -> &'static str {
        match self {
            Self::Discovered { issuer, .. } => issuer.issuer,
            Self::Published(issuer) => issuer.issuer,
        }
    }
}

use std::sync::Arc;

use url::Url;

use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;

pub(crate) enum GroupLocator {
    Discovery {
        discovery_url: Url,
        metadata: Vec<Arc<IssuerMetadata>>,
    },
    Endpoint(Arc<dyn ProvidesEndpoint>),
}

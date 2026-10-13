use margaret_attributes::canonical_path::CanonicalPath;

use crate::provider_endpoint::ProviderEndpoint;

pub(crate) enum OidcEndpointMarker {
    Consent { view: CanonicalPath },
    Endpoint(ProviderEndpoint),
}

impl OidcEndpointMarker {
    pub(crate) fn endpoint(&self) -> ProviderEndpoint {
        match self {
            Self::Consent { .. } => ProviderEndpoint::Consent,
            Self::Endpoint(endpoint) => *endpoint,
        }
    }
}

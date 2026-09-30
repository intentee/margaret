use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_oidc_discovery::metadata_endpoint::MetadataEndpoint;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerEndpoint {
    Authorization,
    Introspection,
    Token,
    Userinfo,
}

impl ServerEndpoint {
    #[must_use]
    pub fn advertised_in(self, metadata: &ProviderMetadata) -> &AdvertisedEndpoint {
        match self {
            Self::Authorization => &metadata.authorization_endpoint,
            Self::Introspection => &metadata.introspection_endpoint,
            Self::Token => &metadata.token_endpoint,
            Self::Userinfo => &metadata.userinfo_endpoint,
        }
    }

    #[must_use]
    pub fn metadata_endpoint(self) -> MetadataEndpoint {
        match self {
            Self::Authorization => MetadataEndpoint::AuthorizationEndpoint,
            Self::Introspection => MetadataEndpoint::IntrospectionEndpoint,
            Self::Token => MetadataEndpoint::TokenEndpoint,
            Self::Userinfo => MetadataEndpoint::UserinfoEndpoint,
        }
    }
}

impl Display for ServerEndpoint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        self.metadata_endpoint().fmt(formatter)
    }
}

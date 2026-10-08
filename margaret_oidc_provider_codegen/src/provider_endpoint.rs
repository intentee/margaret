use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_jwks_codegen::public_jwks_handler_canonical_path::public_jwks_handler_canonical_path;
use margaret_route_method::route_method::RouteMethod;

use crate::oidc_provider_item::OidcProviderItem;
use crate::oidc_provider_item_path::oidc_provider_item_path;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderEndpoint {
    Authorization,
    Discovery,
    Introspection,
    Jwks,
    Revocation,
    Token,
    Userinfo,
}

impl ProviderEndpoint {
    #[must_use]
    pub fn admits(self, method: RouteMethod) -> bool {
        match self {
            Self::Authorization | Self::Userinfo => {
                matches!(method, RouteMethod::Get | RouteMethod::Post)
            }
            Self::Discovery | Self::Jwks => method == RouteMethod::Get,
            Self::Introspection | Self::Revocation | Self::Token => method == RouteMethod::Post,
        }
    }

    #[must_use]
    pub fn handler_path(self) -> CanonicalPath {
        match self {
            Self::Authorization => oidc_provider_item_path(OidcProviderItem::AuthorizationEndpoint),
            Self::Discovery => oidc_provider_item_path(OidcProviderItem::ProviderMetadataHandler),
            Self::Introspection => oidc_provider_item_path(OidcProviderItem::IntrospectionEndpoint),
            Self::Jwks => public_jwks_handler_canonical_path(),
            Self::Revocation => oidc_provider_item_path(OidcProviderItem::RevocationEndpoint),
            Self::Token => oidc_provider_item_path(OidcProviderItem::TokenEndpoint),
            Self::Userinfo => oidc_provider_item_path(OidcProviderItem::UserinfoEndpoint),
        }
    }
}

impl ProviderEndpoint {
    #[must_use]
    pub fn capability(self) -> &'static str {
        match self {
            Self::Authorization => "grants authorization codes",
            Self::Introspection => "introspects tokens",
            Self::Revocation => "is granted refresh tokens",
            Self::Userinfo => "requests the openid scope with authorization codes",
            Self::Discovery | Self::Jwks | Self::Token => "is admitted",
        }
    }
}

impl Display for ProviderEndpoint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(match self {
            Self::Authorization => "authorization endpoint",
            Self::Discovery => "discovery document",
            Self::Introspection => "introspection endpoint",
            Self::Jwks => "jwks document",
            Self::Revocation => "revocation endpoint",
            Self::Token => "token endpoint",
            Self::Userinfo => "userinfo endpoint",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ProviderEndpoint;

    #[test]
    fn describes_the_capability_each_endpoint_serves() {
        assert_eq!(
            [
                ProviderEndpoint::Authorization,
                ProviderEndpoint::Discovery,
                ProviderEndpoint::Introspection,
                ProviderEndpoint::Jwks,
                ProviderEndpoint::Revocation,
                ProviderEndpoint::Token,
                ProviderEndpoint::Userinfo,
            ]
            .map(ProviderEndpoint::capability),
            [
                "grants authorization codes",
                "is admitted",
                "introspects tokens",
                "is admitted",
                "is granted refresh tokens",
                "is admitted",
                "requests the openid scope with authorization codes",
            ]
        );
    }
}

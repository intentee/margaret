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

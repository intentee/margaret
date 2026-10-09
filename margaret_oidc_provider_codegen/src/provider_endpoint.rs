use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_http_codegen::framework_input::FrameworkInput;
use margaret_jwks_codegen::public_jwks_handler_canonical_path::public_jwks_handler_canonical_path;
use margaret_route_method::route_method::RouteMethod;

use crate::authorization_handler_path::authorization_handler_path;
use crate::endpoint_admission::EndpointAdmission;
use crate::oidc_provider_item::OidcProviderItem;
use crate::oidc_provider_item_path::oidc_provider_item_path;

fn admitted_on(
    method: RouteMethod,
    admitted: RouteMethod,
    input: FrameworkInput,
) -> EndpointAdmission {
    if method == admitted {
        EndpointAdmission::Admitted(input)
    } else {
        EndpointAdmission::Refused
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderEndpoint {
    Authorization,
    Consent,
    Discovery,
    Introspection,
    Jwks,
    Revocation,
    Token,
    Userinfo,
}

impl ProviderEndpoint {
    #[must_use]
    pub fn capability(self) -> &'static str {
        match self {
            Self::Authorization | Self::Consent => "grants authorization codes",
            Self::Introspection => "introspects tokens",
            Self::Revocation => "is granted refresh tokens",
            Self::Userinfo => "requests the openid scope with authorization codes",
            Self::Discovery | Self::Jwks | Self::Token => "is admitted",
        }
    }

    #[must_use]
    pub fn handler_path(self) -> CanonicalPath {
        match self {
            Self::Authorization => authorization_handler_path(),
            Self::Consent => oidc_provider_item_path(OidcProviderItem::ConsentHandler),
            Self::Discovery => oidc_provider_item_path(OidcProviderItem::ProviderMetadataHandler),
            Self::Introspection => oidc_provider_item_path(OidcProviderItem::IntrospectionEndpoint),
            Self::Jwks => public_jwks_handler_canonical_path(),
            Self::Revocation => oidc_provider_item_path(OidcProviderItem::RevocationEndpoint),
            Self::Token => oidc_provider_item_path(OidcProviderItem::TokenEndpoint),
            Self::Userinfo => oidc_provider_item_path(OidcProviderItem::UserinfoEndpoint),
        }
    }

    pub(crate) fn admission(self, method: RouteMethod) -> EndpointAdmission {
        match self {
            Self::Authorization => match method {
                RouteMethod::Get => EndpointAdmission::Admitted(FrameworkInput::Head),
                RouteMethod::Post => EndpointAdmission::Admitted(FrameworkInput::Content),
                RouteMethod::Delete
                | RouteMethod::Patch
                | RouteMethod::Put
                | RouteMethod::Query => EndpointAdmission::Refused,
            },
            Self::Userinfo => match method {
                RouteMethod::Get | RouteMethod::Post => {
                    EndpointAdmission::Admitted(FrameworkInput::Head)
                }
                RouteMethod::Delete
                | RouteMethod::Patch
                | RouteMethod::Put
                | RouteMethod::Query => EndpointAdmission::Refused,
            },
            Self::Discovery | Self::Jwks => {
                admitted_on(method, RouteMethod::Get, FrameworkInput::Head)
            }
            Self::Consent | Self::Introspection | Self::Revocation | Self::Token => {
                admitted_on(method, RouteMethod::Post, FrameworkInput::Content)
            }
        }
    }
}

impl Display for ProviderEndpoint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(match self {
            Self::Authorization => "authorization endpoint",
            Self::Consent => "consent endpoint",
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
    use margaret_route_method::route_method::RouteMethod;

    use super::ProviderEndpoint;
    use crate::endpoint_admission::EndpointAdmission;

    const ENDPOINTS: [ProviderEndpoint; 8] = [
        ProviderEndpoint::Authorization,
        ProviderEndpoint::Consent,
        ProviderEndpoint::Discovery,
        ProviderEndpoint::Introspection,
        ProviderEndpoint::Jwks,
        ProviderEndpoint::Revocation,
        ProviderEndpoint::Token,
        ProviderEndpoint::Userinfo,
    ];

    const METHODS: [RouteMethod; 6] = [
        RouteMethod::Delete,
        RouteMethod::Get,
        RouteMethod::Patch,
        RouteMethod::Post,
        RouteMethod::Put,
        RouteMethod::Query,
    ];

    #[test]
    fn describes_the_capability_each_endpoint_serves() {
        assert_eq!(
            ENDPOINTS.map(ProviderEndpoint::capability),
            [
                "grants authorization codes",
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

    #[test]
    fn admits_each_endpoint_only_on_the_methods_it_answers() {
        let admitted = |endpoint: ProviderEndpoint| {
            METHODS
                .into_iter()
                .filter_map(|method| match endpoint.admission(method) {
                    EndpointAdmission::Admitted(input) => Some(format!("{method:?}:{input:?}")),
                    EndpointAdmission::Refused => None,
                })
                .collect::<Vec<String>>()
        };

        assert_eq!(
            ENDPOINTS.map(admitted),
            [
                vec!["Get:Head".to_string(), "Post:Content".to_string()],
                vec!["Post:Content".to_string()],
                vec!["Get:Head".to_string()],
                vec!["Post:Content".to_string()],
                vec!["Get:Head".to_string()],
                vec!["Post:Content".to_string()],
                vec!["Post:Content".to_string()],
                vec!["Get:Head".to_string(), "Post:Head".to_string()],
            ]
        );
    }
}

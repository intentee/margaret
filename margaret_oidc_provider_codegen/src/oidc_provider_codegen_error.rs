use thiserror::Error;

use margaret_route_method::route_method::RouteMethod;

use crate::provider_endpoint::ProviderEndpoint;

#[derive(Debug, Error)]
pub enum OidcProviderCodegenError {
    #[error("the {endpoint} is served at several paths of the server '{server}': {paths:?}")]
    AmbiguousEndpointRoute {
        endpoint: ProviderEndpoint,
        paths: Vec<String>,
        server: String,
    },

    #[error(
        "the discovery document is served at '{path}', but its issuer publishes it at '{expected}'"
    )]
    DiscoveryPathMismatch { expected: String, path: String },

    #[error("the discovery document is served by several servers: {servers:?}")]
    DiscoveryServedBySeveralServers { servers: Vec<String> },

    #[error("the {endpoint} is served by a {method:?} route at '{path}', which it does not admit")]
    EndpointRouteMethod {
        endpoint: ProviderEndpoint,
        method: RouteMethod,
        path: String,
    },

    #[error("no route of the server '{server}' serves the {endpoint}")]
    MissingEndpointRoute {
        endpoint: ProviderEndpoint,
        server: String,
    },

    #[error(
        "no route uses the consent endpoint, so the authorization endpoint cannot ask an end user for consent"
    )]
    MissingConsentRoute,

    #[error("no route serves the discovery document")]
    MissingDiscoveryRoute,

    #[error(
        "a route serves the consent endpoint, but no admitted client grants authorization codes an end user could consent to"
    )]
    UnconsumedConsentRoute,

    #[error("the {endpoint} is routed, but no admitted client {capability}")]
    UnconsumedEndpointRoute {
        capability: &'static str,
        endpoint: ProviderEndpoint,
    },

    #[error("the {endpoint} is routed, but the provider admits no oauth client")]
    EndpointRouteWithoutAdmittedClients { endpoint: ProviderEndpoint },

    #[error("the {endpoint} is served at '{path}', which has route parameters")]
    ParameterizedEndpointRoute {
        endpoint: ProviderEndpoint,
        path: String,
    },
}

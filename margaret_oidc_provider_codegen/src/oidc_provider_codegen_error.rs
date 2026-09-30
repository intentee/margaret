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

    #[error("no route serves the discovery document")]
    MissingDiscoveryRoute,

    #[error("the {endpoint} is served at '{path}', which has route parameters")]
    ParameterizedEndpointRoute {
        endpoint: ProviderEndpoint,
        path: String,
    },
}

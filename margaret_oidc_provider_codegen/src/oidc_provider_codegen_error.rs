use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
use margaret_route_method::route_method::RouteMethod;

use crate::provider_endpoint::ProviderEndpoint;

#[derive(Debug, Error)]
pub enum OidcProviderCodegenError {
    #[error(transparent)]
    Anchor(#[from] DeclarationAnchorError),

    #[error(transparent)]
    AttributeArguments(#[from] AttributeArgumentsError),

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error(
        "#[serves_oidc_endpoint] on '{anchor}' names no endpoint; name a variant of margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint"
    )]
    MissingOidcEndpoint { anchor: String },

    #[error(
        "#[serves_oidc_endpoint] on '{anchor}' names '{written}', which is not a variant of margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint"
    )]
    UnknownOidcEndpoint { anchor: String, written: String },

    #[error(
        "#[serves_oidc_endpoint] on '{anchor}' serves an endpoint, but '{anchor}' responds to no HTTP request; route it with #[responds_to_http]"
    )]
    UnroutedOidcEndpoint { anchor: String },

    #[error(
        "#[serves_oidc_endpoint] on '{anchor}' serves the consent endpoint without a view; name the view that renders the consent page with `view = <View>`"
    )]
    MissingConsentView { anchor: String },

    #[error(
        "#[serves_oidc_endpoint] on '{anchor}' renders the consent page with '{written}', which names no item of the crate"
    )]
    UnknownConsentView { anchor: String, written: String },

    #[error(
        "#[serves_oidc_endpoint] on '{anchor}' renders the consent page with '{view}', which does not declare #[renders_view]"
    )]
    UnrenderedConsentView { anchor: String, view: String },

    #[error(
        "the consent endpoint is served by both '{first}' and '{second}'; the authorization endpoint asks for consent through one route"
    )]
    AmbiguousConsentEndpoint { first: String, second: String },

    #[error(
        "the authorization endpoint is routed, but the crate declares no #[issues_sessions] to authenticate its end users"
    )]
    AuthorizationWithoutSessions,

    #[error(
        "the consent endpoint '{route}' is served by the server '{server}', but the authorization endpoint is served by '{provider_server}'"
    )]
    ConsentRouteOnForeignServer {
        provider_server: String,
        route: String,
        server: String,
    },

    #[error(
        "#[provides_userinfo_claims] is declared by both '{first}' and '{second}'; the userinfo endpoint answers with the claims of one provider"
    )]
    AmbiguousUserinfoClaimsProvider { first: String, second: String },

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

    #[error("no route serves the discovery document")]
    MissingDiscoveryRoute,

    #[error(
        "the userinfo endpoint is routed, but no singleton declares #[provides_userinfo_claims] to answer it"
    )]
    MissingUserinfoClaimsProvider,

    #[error(
        "'{provider}' declares #[provides_userinfo_claims], but no route serves the userinfo endpoint"
    )]
    UnconsumedUserinfoClaimsProvider { provider: String },

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

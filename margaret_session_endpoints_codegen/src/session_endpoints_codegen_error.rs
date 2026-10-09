use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
use margaret_http_codegen::http_codegen_error::HttpCodegenError;
use margaret_route_method::route_method::RouteMethod;

#[derive(Debug, Error)]
pub enum SessionEndpointsCodegenError {
    #[error(transparent)]
    Anchor(#[from] DeclarationAnchorError),

    #[error(transparent)]
    AttributeArguments(#[from] AttributeArgumentsError),

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error(transparent)]
    LandingRoute(#[from] HttpCodegenError),

    #[error("#[serves_session_endpoint] on '{anchor}' names no endpoint")]
    MissingSessionEndpoint { anchor: String },

    #[error(
        "#[serves_session_endpoint] on '{anchor}' names '{written}', which is no variant of margaret::framework::sessions::session_endpoint::SessionEndpoint"
    )]
    UnknownSessionEndpoint { anchor: String, written: String },

    #[error("#[serves_session_endpoint] on '{anchor}' signs out without its `landing_route`")]
    MissingLandingRoute { anchor: String },

    #[error("#[serves_session_endpoint] on '{anchor}' lands on '{written}', which names no item")]
    UnknownLandingRoute { anchor: String, written: String },

    #[error(
        "#[serves_session_endpoint] on '{anchor}' serves an endpoint, but '{anchor}' responds to no HTTP request; route it with #[responds_to_http]"
    )]
    UnroutedSessionEndpoint { anchor: String },

    #[error(
        "#[serves_session_endpoint] on '{anchor}' answers RouteMethod::{method:?}; a session endpoint answers RouteMethod::Post"
    )]
    SessionEndpointMethod { anchor: String, method: RouteMethod },

    #[error(
        "#[serves_session_endpoint] on '{anchor}' serves sessions, but no struct declares #[issues_sessions]"
    )]
    SessionEndpointWithoutIssuedSessions { anchor: String },

    #[error("both '{first}' and '{second}' serve the same session endpoint")]
    AmbiguousSessionEndpoint { first: String, second: String },
}

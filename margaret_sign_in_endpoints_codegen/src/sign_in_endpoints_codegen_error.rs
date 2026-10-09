use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
use margaret_http_codegen::http_codegen_error::HttpCodegenError;
use margaret_route_method::route_method::RouteMethod;

#[derive(Debug, Error)]
pub enum SignInEndpointsCodegenError {
    #[error(transparent)]
    Anchor(#[from] DeclarationAnchorError),

    #[error(transparent)]
    AttributeArguments(#[from] AttributeArgumentsError),

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error(transparent)]
    Route(#[from] HttpCodegenError),

    #[error(
        "#[serves_sign_in] on '{anchor}' names no endpoint; name a variant of margaret::framework::oidc_sign_in::sign_in_endpoint::SignInEndpoint"
    )]
    MissingSignInEndpoint { anchor: String },

    #[error(
        "#[serves_sign_in] on '{anchor}' names '{written}', which is not a variant of margaret::framework::oidc_sign_in::sign_in_endpoint::SignInEndpoint"
    )]
    UnknownSignInEndpoint { anchor: String, written: String },

    #[error("'{anchor}' declares a sign-in without the `client` it signs in through")]
    MissingSignInClient { anchor: String },

    #[error("'{anchor}' declares a sign-in through a `client` that is not a plain tag")]
    MalformedSignInClient { anchor: String },

    #[error(
        "#[serves_sign_in] on '{anchor}' serves a sign-in endpoint, but '{anchor}' responds to no HTTP request; route it with #[responds_to_http]"
    )]
    UnroutedSignInEndpoint { anchor: String },

    #[error(
        "#[serves_sign_in] on '{anchor}' is routed by RouteMethod::{method:?}, but a browser reaches the sign-in endpoints with RouteMethod::Get"
    )]
    SignInEndpointMethod { anchor: String, method: RouteMethod },

    #[error(
        "#[serves_sign_in] on '{anchor}' serves the sign-in callback without a `landing_route` to redirect a signed-in visitor to"
    )]
    MissingLandingRoute { anchor: String },

    #[error(
        "#[serves_sign_in] on '{anchor}' lands on '{written}', which names no item of the crate"
    )]
    UnknownLandingRoute { anchor: String, written: String },

    #[error("'{anchor}' signs in through '{client}', which no #[oauth_client] declares")]
    UnknownSignInClient { anchor: String, client: String },

    #[error(
        "'{anchor}' signs in through '{client}', which declares no sign-in; declare `sign_in(scopes = [...])`, or admit the client with an authorization_code grant redirecting to one of its routes"
    )]
    ClientWithoutSignIn { anchor: String, client: String },

    #[error("the sign-in of '{client}' starts at both '{first}' and '{second}'")]
    AmbiguousSignInStart {
        client: String,
        first: String,
        second: String,
    },

    #[error("the sign-in of '{client}' returns to both '{first}' and '{second}'")]
    AmbiguousSignInCallback {
        client: String,
        first: String,
        second: String,
    },

    #[error("the sign-in of '{client}' is admitted by both '{first}' and '{second}'")]
    AmbiguousSignInAdmission {
        client: String,
        first: String,
        second: String,
    },

    #[error(
        "the oauth client '{client}' signs in, but no route serves SignInEndpoint::Start for it"
    )]
    MissingSignInStart { client: String },

    #[error(
        "the oauth client '{client}' signs in, but no route serves SignInEndpoint::Callback for it"
    )]
    MissingSignInCallback { client: String },

    #[error(
        "the oauth client '{client}' signs in, but no singleton declares #[admits_sign_in(client = {client})] to admit its identities"
    )]
    MissingSignInAdmission { client: String },

    #[error(
        "the sign-in of '{client}' returns to '{route}', which its admitted client does not list among its redirect_routes"
    )]
    UnregisteredSignInCallback { client: String, route: String },

    #[error(
        "the oauth client '{client}' signs in, but the crate declares no #[issues_sessions] to start the sessions of its visitors"
    )]
    SignInWithoutIssuedSessions { client: String },
}

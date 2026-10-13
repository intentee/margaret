use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;

#[derive(Debug, Error)]
pub enum SessionsCodegenError {
    #[error(transparent)]
    Anchor(#[from] DeclarationAnchorError),

    #[error(transparent)]
    AttributeArguments(#[from] AttributeArgumentsError),

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error(
        "both '{first}' and '{second}' declare the sessions of the application; a crate issues or consumes sessions through one declaration"
    )]
    AmbiguousSessions { first: String, second: String },

    #[error("'{anchor}' declares sessions without the `issuer` tag they belong to")]
    MissingSessionsIssuer { anchor: String },

    #[error("'{anchor}' declares sessions whose `issuer` is not a plain tag")]
    MalformedSessionsIssuer { anchor: String },

    #[error(
        "#[issues_sessions] on '{anchor}' declares no `audience` for its session access tokens"
    )]
    MissingSessionAudience { anchor: String },

    #[error("#[issues_sessions] on '{anchor}' declares an empty `audience`")]
    EmptySessionAudience { anchor: String },

    #[error(
        "#[issues_sessions] on '{anchor}' addresses session access tokens to '{audience}', which names their own issuer, so the tokens the provider addresses to the issuer would pass as sessions"
    )]
    SessionAudienceNamesIssuer { anchor: String, audience: String },

    #[error(
        "#[issues_sessions] on '{anchor}' addresses session access tokens to '{audience}', which the resource '{resource}' also uses; a session access token must never be accepted as a resource token"
    )]
    SessionAudienceOfResource {
        anchor: String,
        audience: String,
        resource: String,
    },

    #[error("#[issues_sessions] on '{anchor}' declares no `cookies` policy")]
    MissingSessionCookies { anchor: String },

    #[error(
        "#[issues_sessions] on '{anchor}' declares the cookies '{written}', which is no variant of margaret::framework::sessions::session_cookies::SessionCookies"
    )]
    UnknownSessionCookies { anchor: String, written: String },

    #[error(
        "#[issues_sessions] on '{anchor}' shares session cookies without the `domain_from` environment variable"
    )]
    MissingCookieDomainSource { anchor: String },

    #[error(
        "#[issues_sessions] on '{anchor}' reads the session cookie domain from '{name}', which is not an environment variable name"
    )]
    MalformedCookieDomainSource { anchor: String, name: String },

    #[error(
        "#[consumes_sessions] on '{anchor}' declares no `cookie_domain_from` environment variable"
    )]
    MissingConsumedCookieDomainSource { anchor: String },

    #[error(
        "#[consumes_sessions] on '{anchor}' reads the session cookie domain from '{name}', which is not an environment variable name"
    )]
    MalformedConsumedCookieDomainSource { anchor: String, name: String },

    #[error(
        "#[consumes_sessions] on '{anchor}' declares no `refresh_url_from` environment variable"
    )]
    MissingRefreshUrlSource { anchor: String },

    #[error(
        "#[consumes_sessions] on '{anchor}' reads the session refresh url from '{name}', which is not an environment variable name"
    )]
    MalformedRefreshUrlSource { anchor: String, name: String },

    #[error(
        "#[consumes_sessions] on '{anchor}' reads both the cookie domain and the refresh url from '{name}'"
    )]
    SharedEnvironmentVariable { anchor: String, name: String },
}

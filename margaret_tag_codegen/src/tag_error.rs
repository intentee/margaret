use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;

use crate::tag_expectation::TagExpectation;
use crate::tag_kind::TagKind;

#[derive(Debug, Error)]
pub enum TagError {
    #[error("failed to read the attribute arguments: {source}")]
    AttributeArguments {
        #[from]
        source: AttributeArgumentsError,
    },

    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error("the middleware handler '{concrete}' does not name a tag")]
    MissingMiddlewareTag { concrete: String },

    #[error("the middleware handler '{concrete}' names a tag that is not a single plain name")]
    MalformedMiddlewareTag { concrete: String },

    #[error("the tag '{tag}' is declared more than once: by '{first}' and by '{second}'")]
    DuplicateTag {
        tag: String,
        first: String,
        second: String,
    },

    #[error("{site} references the tag '{tag}', which no {kind} declares")]
    UnknownTag {
        site: String,
        tag: String,
        kind: TagExpectation,
    },

    #[error("{site} references the tag '{tag}', which is a {found}, not a {expected}")]
    WrongKind {
        site: String,
        tag: String,
        expected: TagExpectation,
        found: TagKind,
    },

    #[error("{site} must reference exactly one tag by its plain name")]
    MalformedReference { site: String },

    #[error("{site} must be `issuer = <tag>`")]
    MalformedBearerToken { site: String },

    #[error("{site} must be `client = <tag>`")]
    MalformedIntrospectedBearerToken { site: String },

    #[error(
        "the oauth clients '{first}' and '{second}' both identify as '{client_id}' at the issuer '{issuer}', so the issuer cannot tell them apart"
    )]
    DuplicateOAuthClient {
        client_id: String,
        first: String,
        issuer: String,
        second: String,
    },

    #[error("{site} names the issuer '{issuer}', which publishes no discovery document")]
    OAuthClientIssuerNotDiscovered { issuer: String, site: String },

    #[error("the subject token exchanger '{concrete}' must name its issuer as `issuer = <tag>`")]
    MalformedSubjectTokenExchangerIssuer { concrete: String },

    #[error(
        "the issuer '{issuer}' has more than one subject token exchanger: '{first}' and '{second}'"
    )]
    DuplicateSubjectTokenExchanger {
        issuer: String,
        first: String,
        second: String,
    },
}

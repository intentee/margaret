use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;

#[derive(Debug, Error)]
pub enum TokenIssuanceCodegenError {
    #[error(
        "#[issues_tokens] is declared by both '{first}' and '{second}'; an application issues tokens under one issuer"
    )]
    AmbiguousTokenIssuance { first: String, second: String },

    #[error(transparent)]
    Anchor(#[from] DeclarationAnchorError),

    #[error(transparent)]
    AttributeArguments(#[from] AttributeArgumentsError),

    #[error(
        "the resource audience '{audience}' is declared by both '{first}' and '{second}', so its tokens have no single resource"
    )]
    DuplicateResourceAudience {
        audience: String,
        first: String,
        second: String,
    },

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error("the issuer '#[issues_tokens]' declares on '{anchor}' is malformed: {source}")]
    MalformedIssuer {
        anchor: String,
        #[source]
        source: RegisteredClaimsError,
    },

    #[error("#[issues_resource_tokens] on '{anchor}' declares an empty audience")]
    EmptyResourceAudience { anchor: String },

    #[error("#[issues_resource_tokens] on '{anchor}' names a tag that is not a single plain name")]
    MalformedResourceTag { anchor: String },

    #[error("#[issues_tokens] on '{anchor}' names a tag that is not a single plain name")]
    MalformedTag { anchor: String },

    #[error("#[issues_tokens] on '{anchor}' does not declare the issuer of the tokens")]
    MissingIssuer { anchor: String },

    #[error("#[issues_resource_tokens] on '{anchor}' does not declare the audience of the tokens")]
    MissingResourceAudience { anchor: String },

    #[error("#[issues_resource_tokens] on '{anchor}' does not name a tag")]
    MissingResourceTag { anchor: String },

    #[error("#[issues_tokens] on '{anchor}' does not name a tag")]
    MissingTag { anchor: String },

    #[error(
        "#[issues_resource_tokens] on '{anchor}' declares the audience '{audience}', which names the issuer, so its tokens would pass where the issuer is addressed"
    )]
    ResourceAudienceNamesIssuer { anchor: String, audience: String },

    #[error(
        "#[issues_resource_tokens] on '{anchor}' declares resource tokens, but no #[issues_tokens] declares the issuer that signs them"
    )]
    ResourceWithoutTokenIssuance { anchor: String },
}

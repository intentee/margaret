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

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error("the audience '#[issues_tokens]' declares on '{anchor}' is malformed: {source}")]
    MalformedAudience {
        anchor: String,
        #[source]
        source: RegisteredClaimsError,
    },

    #[error("the issuer '#[issues_tokens]' declares on '{anchor}' is malformed: {source}")]
    MalformedIssuer {
        anchor: String,
        #[source]
        source: RegisteredClaimsError,
    },

    #[error("#[issues_tokens] on '{anchor}' does not declare the audience of the tokens")]
    MissingAudience { anchor: String },

    #[error("#[issues_tokens] on '{anchor}' does not declare the issuer of the tokens")]
    MissingIssuer { anchor: String },
}

use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
use margaret_oauth_vocabulary::scope_rejection::ScopeRejection;

#[derive(Debug, Error)]
pub enum OAuthVocabularyCodegenError {
    #[error(transparent)]
    Anchor(#[from] DeclarationAnchorError),

    #[error(transparent)]
    AttributeArguments(#[from] AttributeArgumentsError),

    #[error(
        "#[oauth_scope] declares the scope '{name}' on both '{first}' and '{second}'; every scope is declared once"
    )]
    DuplicateScopeName {
        first: String,
        name: String,
        second: String,
    },

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error("the scope #[oauth_scope] declares on '{anchor}' is malformed: {rejection}")]
    MalformedScopeName {
        anchor: String,
        rejection: ScopeRejection,
    },

    #[error("#[oauth_scope] on '{anchor}' declares no scope name")]
    MissingScopeName { anchor: String },

    #[error(
        "#[oauth_scope] on '{anchor}' declares the openid scope, which the framework declares as margaret::framework::oauth_vocabulary::openid_scope::OpenidScope"
    )]
    RedeclaredOpenidScope { anchor: String },

    #[error(
        "'{anchor}' declares the scope '{name}' with #[oauth_scope], but no client declaration grants or requests it"
    )]
    UnconsumedScope { anchor: String, name: String },
}

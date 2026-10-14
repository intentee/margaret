use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;

#[derive(Debug, Error)]
pub enum JwksCodegenError {
    #[error(
        "#[encrypts_signing_keys] is declared by both '{first}' and '{second}'; an application seals its signing keys under one key"
    )]
    AmbiguousSigningKeysEncryption { first: String, second: String },

    #[error(transparent)]
    Anchor(#[from] DeclarationAnchorError),

    #[error(transparent)]
    AttributeArguments(#[from] AttributeArgumentsError),

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error(
        "#[encrypts_signing_keys] on '{anchor}' reads its key from '{name}', which is not an environment variable name"
    )]
    MalformedKeySource { anchor: String, name: String },

    #[error(
        "#[encrypts_signing_keys] on '{anchor}' declares no key_from environment variable to read the signing keys encryption key from"
    )]
    MissingKeySource { anchor: String },
}

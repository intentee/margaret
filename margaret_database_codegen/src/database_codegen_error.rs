use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;

#[derive(Debug, Error)]
pub enum DatabaseCodegenError {
    #[error(
        "#[postgres_database] is declared by both '{first}' and '{second}'; an application keeps its data in one database"
    )]
    AmbiguousPostgresDatabase { first: String, second: String },

    #[error(transparent)]
    Anchor(#[from] DeclarationAnchorError),

    #[error(transparent)]
    AttributeArguments(#[from] AttributeArgumentsError),

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error(
        "#[postgres_database] on '{anchor}' reads its url from '{name}', which is not an environment variable name"
    )]
    MalformedUrlSource { anchor: String, name: String },

    #[error(
        "#[postgres_database] on '{anchor}' declares no url_from environment variable to read its url from"
    )]
    MissingUrlSource { anchor: String },
}

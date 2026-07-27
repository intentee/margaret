use thiserror::Error;

#[derive(Debug, Error)]
pub enum AttributeArgsParseError {
    #[error("arguments of attribute '{attribute_path}' could not be parsed: {source}")]
    Malformed {
        attribute_path: String,
        #[source]
        source: syn::Error,
    },

    #[error("argument '{key}' of attribute '{attribute_path}' is provided more than once")]
    DuplicateNamedArgument { attribute_path: String, key: String },
}

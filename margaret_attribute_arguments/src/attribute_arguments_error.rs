use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AttributeArgumentsError {
    #[error(
        "argument '{key}' of attribute '{attribute_path}' lists the element '{element}' more than once"
    )]
    DuplicateArrayElement {
        attribute_path: String,
        element: String,
        key: String,
    },

    #[error("group '{group}' of attribute '{attribute_path}' is provided more than once")]
    DuplicateGroup {
        attribute_path: String,
        group: String,
    },

    #[error("argument '{key}' of attribute '{attribute_path}' is provided more than once")]
    DuplicateNamedArgument { attribute_path: String, key: String },

    #[error("arguments of attribute '{attribute_path}' could not be parsed: {source}")]
    Malformed {
        attribute_path: String,
        #[source]
        source: syn::Error,
    },

    #[error(
        "argument '{key}' of attribute '{attribute_path}' is not a valid unsigned integer: {source}"
    )]
    MalformedUnsignedInteger {
        attribute_path: String,
        key: String,
        #[source]
        source: syn::Error,
    },

    #[error("argument '{key}' of attribute '{attribute_path}' is not a {expected}")]
    UnexpectedArgument {
        attribute_path: String,
        expected: String,
        key: String,
    },

    #[error("attribute '{attribute_path}' has an unrecognized argument '{argument}'")]
    UnrecognizedArgument {
        argument: String,
        attribute_path: String,
    },
}

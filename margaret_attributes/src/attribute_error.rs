use std::io;

use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;

#[derive(Debug, Error)]
pub enum AttributeError {
    #[error(transparent)]
    Arguments(#[from] AttributeArgumentsError),

    #[error("two items resolve to the same canonical path '{path}'")]
    DuplicateCanonicalPath { path: String },

    #[error("failed to parse Rust source file '{path}': {source}")]
    FileParse {
        path: String,
        #[source]
        source: syn::Error,
    },

    #[error("failed to read Rust source file '{path}': {source}")]
    FileRead {
        path: String,
        #[source]
        source: io::Error,
    },

    #[error("glob import is not allowed in '{file}'")]
    GlobImport { file: String },

    #[error("invalid attribute selector '{input}': {source}")]
    InvalidSelector {
        input: String,
        #[source]
        source: syn::Error,
    },

    #[error("module '{module}' resolves to both '{file_module}' and '{directory_module}'")]
    ModuleFileCollision {
        module: String,
        file_module: String,
        directory_module: String,
    },

    #[error("module '{module}' has no source file at '{file_module}' or '{directory_module}'")]
    ModuleFileNotFound {
        module: String,
        file_module: String,
        directory_module: String,
    },

    #[error("module '{module}' in '{file}' uses a #[path] attribute, which is not allowed")]
    ModulePathAttribute { module: String, file: String },

    #[error(
        "attribute '{attribute_path}' is repeated on '{target}' but a single occurrence was expected"
    )]
    RepeatedAttribute {
        attribute_path: String,
        target: String,
    },
}

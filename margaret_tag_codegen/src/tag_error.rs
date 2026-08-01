use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;

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

    #[error("the {kind} '{concrete}' does not name a tag")]
    MissingTag { concrete: String, kind: TagKind },

    #[error("the {kind} '{concrete}' names a tag that is not a single plain name")]
    MalformedTag { concrete: String, kind: TagKind },

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
        kind: TagKind,
    },

    #[error("{site} references the tag '{tag}', which is a {found}, not a {expected}")]
    WrongKind {
        site: String,
        tag: String,
        expected: TagKind,
        found: TagKind,
    },

    #[error("{site} must reference exactly one tag by its plain name")]
    MalformedReference { site: String },

    #[error("{site} must be either `server` or `client = <tag>`")]
    MalformedJwksSecretStore { site: String },
}

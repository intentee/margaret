use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;

#[derive(Debug, Error)]
pub enum ActiveRecordCodegenError {
    #[error("failed to read the attribute arguments: {source}")]
    AttributeArguments {
        #[from]
        source: AttributeArgumentsError,
    },

    #[error(
        "field '{field}' of shape '{shape}' holds the base model as '{found}', which is not the model '{model}'"
    )]
    BaseTypeMismatch {
        field: String,
        found: String,
        model: String,
        shape: String,
    },

    #[error("shape '{shape}' declares the relation '{relation}' more than once")]
    DuplicateShapeRelation { relation: String, shape: String },

    #[error("'{shape}' is annotated with #[eager_load] but is not a struct")]
    EagerLoadNotAStruct { shape: String },

    #[error("shape '{shape}' names '{model}', which is not a #[model] of this crate")]
    EagerLoadModelNotAModel { model: String, shape: String },

    #[error("shape '{shape}' does not name the model it loads")]
    EagerLoadRequiresModel { shape: String },

    #[error("relation '{relation}' of shape '{shape}' has many related rows and requires a limit")]
    HasManyRelationRequiresLimit { relation: String, shape: String },

    #[error("failed to read the shape attributes: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error("relation '{relation}' of shape '{shape}' loads at most one row and takes no limit")]
    LimitOnSingleRelation { relation: String, shape: String },

    #[error(
        "relation '{relation}' of shape '{shape}' declares a limit of {limit}, beyond the largest row count Postgres accepts in LIMIT"
    )]
    RelationLimitExceedsMaximum {
        limit: usize,
        relation: String,
        shape: String,
    },

    #[error("relation '{relation}' of shape '{shape}' declares a limit of zero")]
    RelationLimitMustBePositive { relation: String, shape: String },

    #[error(
        "field '{field}' of shape '{shape}' names the relation '{relation}', which is not a plain identifier"
    )]
    RelationNameIsNotAnIdentifier {
        field: String,
        relation: String,
        shape: String,
    },

    #[error("field '{field}' of shape '{shape}' does not name its relation")]
    RelationRequiresName { field: String, shape: String },

    #[error(
        "field '{field}' of shape '{shape}' holds '{found}' where the relation '{relation}' loads '{expected}'"
    )]
    RelationTypeMismatch {
        expected: String,
        field: String,
        found: String,
        relation: String,
        shape: String,
    },

    #[error(
        "shape '{shape}' declares {count} #[base] fields; a shape loads exactly one base model"
    )]
    ShapeBaseCount { count: usize, shape: String },

    #[error("field '{field}' of shape '{shape}' is marked both #[base] and #[relation]")]
    ShapeFieldMarkedTwice { field: String, shape: String },

    #[error("field '{field}' of shape '{shape}' is neither #[base] nor a #[relation]")]
    ShapeFieldUnmarked { field: String, shape: String },

    #[error(
        "shape '{shape}' has a positional field at index {position}; a shape has named fields only"
    )]
    ShapeRequiresNamedFields { position: usize, shape: String },

    #[error(
        "relation '{relation}' of shape '{shape}' is not a key field or a declared relation of '{model}'"
    )]
    UnknownRelation {
        model: String,
        relation: String,
        shape: String,
    },
}

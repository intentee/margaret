use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;

#[derive(Debug, Error)]
pub enum ModelCodegenError {
    #[error("failed to read the model attributes: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error("'{model}' is annotated with #[model] but is not a struct")]
    ModelNotAStruct { model: String },

    #[error("'{model}' has a duplicate #[model] attribute")]
    DuplicateModelDeclaration { model: String },

    #[error("model '{model}' is missing the required 'table' argument")]
    MissingTable { model: String },

    #[error(
        "model '{model}' has an invalid table name '{table}'; it must be a snake_case identifier"
    )]
    InvalidTableName { model: String, table: String },

    #[error("duplicate table name '{table}' declared by both '{first}' and '{second}'")]
    DuplicateTableName {
        first: String,
        second: String,
        table: String,
    },

    #[error("field '{field}' of model '{model}' has no #[column] attribute")]
    UnattributedField { field: String, model: String },

    #[error(
        "the positional field at index {position} of model '{model}' requires an explicit column name"
    )]
    PositionalColumnRequiresName { model: String, position: usize },

    #[error(
        "model '{model}' has an invalid column name '{column}'; it must be a snake_case identifier"
    )]
    InvalidColumnName { column: String, model: String },

    #[error("model '{model}' has a duplicate column name '{column}'")]
    DuplicateColumnName { column: String, model: String },

    #[error(
        "column '{column}' of model '{model}' has the type '{rust_type}', which cannot be mapped to an SQL type"
    )]
    UninferrableColumnType {
        column: String,
        model: String,
        rust_type: String,
    },

    #[error(
        "foreign key field '{field}' of model '{model}' must also carry a #[column] attribute; #[foreign_key] layers on top of #[column]"
    )]
    ForeignKeyRequiresColumn { field: String, model: String },

    #[error(
        "foreign key field '{field}' of model '{model}' must not set a column name; foreign key column names are derived"
    )]
    ForeignKeyColumnNameIsDerived { field: String, model: String },

    #[error("foreign key field '{field}' of model '{model}' cannot be a primary key")]
    ForeignKeyCannotBePrimaryKey { field: String, model: String },

    #[error(
        "the positional foreign key field at index {position} of model '{model}' requires a named field"
    )]
    ForeignKeyRequiresNamedField { model: String, position: usize },

    #[error(
        "foreign key field '{field}' of model '{model}' has the type '{rust_type}', which is not a #[model]"
    )]
    ForeignKeyTargetNotAModel {
        field: String,
        model: String,
        rust_type: String,
    },

    #[error(
        "foreign key field '{field}' of model '{model}' references '{target}', which has no primary key"
    )]
    ForeignKeyTargetWithoutPrimaryKey {
        field: String,
        model: String,
        target: String,
    },

    #[error(
        "model '{model}' has a table name '{table}' of {length} bytes, exceeding the 63-byte PostgreSQL identifier limit"
    )]
    TableNameTooLong {
        length: usize,
        model: String,
        table: String,
    },

    #[error(
        "model '{model}' has a column name '{column}' of {length} bytes, exceeding the 63-byte PostgreSQL identifier limit"
    )]
    ColumnNameTooLong {
        column: String,
        length: usize,
        model: String,
    },

    #[error(
        "foreign key field '{field}' of model '{model}' has an unknown ON DELETE action '{action}'; valid actions are cascade, restrict, set_null, set_default"
    )]
    UnknownOnDeleteAction {
        action: String,
        field: String,
        model: String,
    },
}

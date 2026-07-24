use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;
use margaret_schema_identifier_naming::schema_identifier_naming_error::SchemaIdentifierNamingError;

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
        "foreign key field '{field}' of model '{model}' references '{target}', which has a composite primary key; foreign keys are single-column"
    )]
    ForeignKeyTargetHasCompositePrimaryKey {
        field: String,
        model: String,
        target: String,
    },

    #[error(
        "foreign key dependency cycle detected between tables: {path}; inline foreign keys require an acyclic table order"
    )]
    ForeignKeyCycle { path: String },

    #[error("model '{model}' has a table name that is too long: {source}")]
    TableNameTooLong {
        model: String,
        #[source]
        source: SchemaIdentifierNamingError,
    },

    #[error("model '{model}' has a column name that is too long: {source}")]
    ColumnNameTooLong {
        model: String,
        #[source]
        source: SchemaIdentifierNamingError,
    },

    #[error(
        "foreign key field '{field}' of model '{model}' derives a column name that is too long: {source}"
    )]
    ForeignKeyColumnNameTooLong {
        field: String,
        model: String,
        #[source]
        source: SchemaIdentifierNamingError,
    },

    #[error(
        "foreign key field '{field}' of model '{model}' has an unknown ON DELETE action '{action}'; valid actions are cascade, restrict, set_null, set_default"
    )]
    UnknownOnDeleteAction {
        action: String,
        field: String,
        model: String,
    },

    #[error(
        "field '{field}' of model '{model}' has an #[index] attribute but no #[column]; #[index] layers on top of #[column]"
    )]
    IndexRequiresColumn { field: String, model: String },

    #[error(
        "field '{field}' of model '{model}' carries a repeated #[index]; a bare #[index] may appear at most once and each index name at most once per column"
    )]
    RepeatedIndexOnColumn { field: String, model: String },

    #[error(
        "model '{model}' has an invalid index name '{index}'; it must be a snake_case identifier"
    )]
    InvalidIndexName { index: String, model: String },

    #[error("model '{model}' derives an index name that is too long: {source}")]
    IndexNameTooLong {
        model: String,
        #[source]
        source: SchemaIdentifierNamingError,
    },

    #[error("model '{model}' declares an index name '{index}' that is too long: {source}")]
    ExplicitIndexNameTooLong {
        index: String,
        model: String,
        #[source]
        source: SchemaIdentifierNamingError,
    },

    #[error("duplicate index name '{name}' declared by tables '{first}' and '{second}'")]
    DuplicateIndexName {
        first: String,
        name: String,
        second: String,
    },

    #[error(
        "index name '{name}' on table '{index_table}' collides with the table name declared by model '{table_model}'"
    )]
    IndexNameCollidesWithTableName {
        index_table: String,
        name: String,
        table_model: String,
    },

    #[error(
        "the constraint-backing index '{name}' generated for table '{constraint_table}' collides with the table name declared by model '{table_model}'"
    )]
    ConstraintIndexCollidesWithTableName {
        constraint_table: String,
        name: String,
        table_model: String,
    },

    #[error(
        "constraint-backing index name '{name}' is generated for both table '{first_table}' and table '{second_table}'"
    )]
    DuplicateConstraintIndexName {
        first_table: String,
        name: String,
        second_table: String,
    },

    #[error(
        "index name '{name}' on table '{index_table}' collides with the constraint-backing index generated for table '{constraint_table}'"
    )]
    IndexNameCollidesWithConstraintIndex {
        constraint_table: String,
        index_table: String,
        name: String,
    },

    #[error(
        "the single-column index on column '{column}' of model '{model}' is redundant; a unique constraint is already indexed"
    )]
    RedundantIndexOnUniqueColumn { column: String, model: String },

    #[error(
        "the single-column index on column '{column}' of model '{model}' is redundant; it is the leading column of the primary key, which is already indexed"
    )]
    RedundantIndexOnPrimaryKeyColumn { column: String, model: String },
}

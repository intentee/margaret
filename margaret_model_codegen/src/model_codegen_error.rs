use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;
use margaret_model::column_type::ColumnType;
use margaret_schema_identifier_naming::schema_identifier_naming_error::SchemaIdentifierNamingError;

use crate::object_kind::ObjectKind;

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
        "foreign key field '{field}' of model '{model}' is missing the required 'name' argument"
    )]
    ForeignKeyMissingName { field: String, model: String },

    #[error(
        "model '{model}' has an invalid foreign key name '{name}' on field '{field}'; it must be a snake_case identifier"
    )]
    InvalidForeignKeyName {
        field: String,
        model: String,
        name: String,
    },

    #[error(
        "foreign key '{name}' on field '{field}' of model '{model}' has a name that is too long: {source}"
    )]
    ForeignKeyNameTooLong {
        field: String,
        model: String,
        name: String,
        #[source]
        source: SchemaIdentifierNamingError,
    },

    #[error(
        "foreign key field '{field}' of model '{model}' is missing the required 'references' argument"
    )]
    ForeignKeyMissingReferences { field: String, model: String },

    #[error(
        "foreign key field '{field}' of model '{model}' references '{reference}', which does not name a column of the target model; use '<Model>::<field>'"
    )]
    ForeignKeyReferenceMissingField {
        field: String,
        model: String,
        reference: String,
    },

    #[error(
        "foreign key field '{field}' of model '{model}' references '{reference}', whose target is not a #[model]"
    )]
    ForeignKeyTargetNotAModel {
        field: String,
        model: String,
        reference: String,
    },

    #[error(
        "foreign key field '{field}' of model '{model}' references '{reference}', but '{target}' has no such column"
    )]
    ForeignKeyUnknownReferencedField {
        field: String,
        model: String,
        reference: String,
        target: String,
    },

    #[error(
        "foreign key '{name}' of model '{model}' references columns of '{target}' that are not a primary key or unique constraint"
    )]
    ForeignKeyReferencedColumnsNotAKey {
        model: String,
        name: String,
        target: String,
    },

    #[error(
        "foreign key '{name}' of model '{model}' references more than one target model; every member of a foreign key must reference the same model"
    )]
    ForeignKeyAmbiguousTarget { model: String, name: String },

    #[error(
        "foreign key field '{field}' of model '{model}' has the local type '{local_type:?}', which does not match the referenced type '{referenced_type:?}'"
    )]
    ForeignKeyColumnTypeMismatch {
        field: String,
        local_type: ColumnType,
        model: String,
        referenced_type: ColumnType,
    },

    #[error(
        "foreign key '{name}' of model '{model}' references the same target column more than once"
    )]
    ForeignKeyDuplicateReferencedColumn { model: String, name: String },

    #[error(
        "foreign key '{name}' of model '{model}' declares more than one distinct ON DELETE action across its columns"
    )]
    InconsistentForeignKeyOnDelete { model: String, name: String },

    #[error(
        "foreign key '{name}' of model '{model}' uses ON DELETE {action}, which nulls its columns on delete, but not all of its columns are nullable"
    )]
    ForeignKeyNullingActionRequiresNullableColumns {
        action: String,
        model: String,
        name: String,
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

    #[error("table '{table}' derives a primary key index name that is too long: {source}")]
    PrimaryKeyIndexNameTooLong {
        #[source]
        source: SchemaIdentifierNamingError,
        table: String,
    },

    #[error("table '{table}' derives a unique index name that is too long: {source}")]
    UniqueIndexNameTooLong {
        #[source]
        source: SchemaIdentifierNamingError,
        table: String,
    },

    #[error(
        "the index over columns '{columns}' of model '{model}' is redundant; its columns are a leading prefix of the primary key, which is already indexed"
    )]
    RedundantIndexOnPrimaryKeyPrefix { columns: String, model: String },

    #[error(
        "the index over columns '{columns}' of model '{model}' is redundant; its columns are a leading prefix of a unique constraint, which is already indexed"
    )]
    RedundantIndexOnUniquePrefix { columns: String, model: String },

    #[error(
        "the {first_kind} named '{name}' on table '{first_table}' collides with the {second_kind} of table '{second_table}'; schema-wide relation names must be unique"
    )]
    RelationNameCollision {
        first_kind: ObjectKind,
        first_table: String,
        name: String,
        second_kind: ObjectKind,
        second_table: String,
    },

    #[error(
        "the {first_kind} named '{name}' on table '{table}' collides with the {second_kind} on the same table; constraint names must be unique per table"
    )]
    ConstraintNameCollision {
        first_kind: ObjectKind,
        name: String,
        second_kind: ObjectKind,
        table: String,
    },
}

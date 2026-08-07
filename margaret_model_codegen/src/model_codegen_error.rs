use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_schema_identifier_naming::schema_identifier_naming_error::SchemaIdentifierNamingError;

#[derive(Debug, Error)]
pub enum ModelCodegenError {
    #[error("failed to read the attribute arguments: {source}")]
    AttributeArguments {
        #[from]
        source: AttributeArgumentsError,
    },

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
        "column '{column}' of model '{model}' has the type '{rust_type}', which resolves to '{resolved_type}' declared in this crate but is not a fieldless enum; a crate type becomes a column only as a fieldless enum, or through #[foreign_key] when it is a #[model]"
    )]
    LocalColumnTypeIsNotAnEnum {
        column: String,
        model: String,
        resolved_type: String,
        rust_type: String,
    },

    #[error(
        "field '{field}' of model '{model}' declares a NUMERIC scale without a precision; both are required"
    )]
    NumericPrecisionMissing { field: String, model: String },

    #[error(
        "field '{field}' of model '{model}' declares a NUMERIC precision without a scale; both are required"
    )]
    NumericScaleMissing { field: String, model: String },

    #[error(
        "field '{field}' of model '{model}' declares the NUMERIC precision {precision}; the precision must be between 1 and 28, because rust_decimal::Decimal stores a 96-bit mantissa whose largest value is 79228162514264337593543950335"
    )]
    NumericPrecisionOutOfRange {
        field: String,
        model: String,
        precision: u32,
    },

    #[error(
        "field '{field}' of model '{model}' declares the NUMERIC scale {scale}, which exceeds its precision {precision}"
    )]
    NumericScaleExceedsPrecision {
        field: String,
        model: String,
        precision: u32,
        scale: u32,
    },

    #[error(
        "column '{column}' of model '{model}' is a rust_decimal::Decimal and requires an explicit precision and scale, e.g. #[column(precision = 12, scale = 2)]"
    )]
    NumericColumnRequiresDigits { column: String, model: String },

    #[error(
        "column '{column}' of model '{model}' declares a NUMERIC precision and scale but has the type '{rust_type}'; only a rust_decimal::Decimal column is sized that way"
    )]
    NumericDigitsOnNonNumericColumn {
        column: String,
        model: String,
        rust_type: String,
    },

    #[error(
        "column '{column}' of model '{model}' maps to enum '{enum_type}', which has no variants; an enum column requires at least one variant"
    )]
    EmptyEnumColumn {
        column: String,
        enum_type: String,
        model: String,
    },

    #[error(
        "column '{column}' of model '{model}' maps to enum '{enum_type}', whose variant '{variant}' carries data; only fieldless (unit) variants are supported"
    )]
    EnumColumnVariantNotUnit {
        column: String,
        enum_type: String,
        model: String,
        variant: String,
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
        "foreign key field '{field}' of model '{model}' must not declare a NUMERIC precision or scale; a foreign key column takes the type of the primary key it references"
    )]
    ForeignKeyCannotDeclareNumericDigits { field: String, model: String },

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

    #[error(
        "self-referential foreign key field '{field}' of model '{model}' must be placed behind heap indirection to break its infinite type size; wrap it in Box, Rc, or Arc, e.g. `Option<Box<...>>`"
    )]
    SelfReferentialForeignKeyRequiresIndirection { field: String, model: String },

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

    #[error("model '{model}' derives a primary key index name that is too long: {source}")]
    PrimaryKeyIndexNameTooLong {
        model: String,
        #[source]
        source: SchemaIdentifierNamingError,
    },

    #[error("model '{model}' derives a unique index name that is too long: {source}")]
    UniqueIndexNameTooLong {
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

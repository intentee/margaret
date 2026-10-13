use thiserror::Error;

use margaret_database::database_error::DatabaseError;

use crate::statement_kind::StatementKind;

#[derive(Debug, Error)]
pub enum ActiveRecordError {
    #[error("the {statement:?} statement on table '{table}' violates a check constraint: {source}")]
    CheckViolation {
        #[source]
        source: DatabaseError,
        statement: StatementKind,
        table: &'static str,
    },

    #[error("the {statement:?} statement on table '{table}' failed: {source}")]
    Database {
        #[source]
        source: DatabaseError,
        statement: StatementKind,
        table: &'static str,
    },

    #[error(
        "the {statement:?} statement on table '{table}' was chosen as a deadlock victim: {source}"
    )]
    Deadlock {
        #[source]
        source: DatabaseError,
        statement: StatementKind,
        table: &'static str,
    },

    #[error("the {statement:?} statement on table '{table}' violates a foreign key: {source}")]
    ForeignKeyViolation {
        #[source]
        source: DatabaseError,
        statement: StatementKind,
        table: &'static str,
    },

    #[error(
        "parameter {position} bound for table '{table}' cannot be serialized to JSON: {source}"
    )]
    JsonSerialization {
        position: usize,
        #[source]
        source: serde_json::Error,
        table: &'static str,
    },

    #[error(
        "column {position} of a row read from table '{table}' holds a value that does not decode: {source}"
    )]
    MalformedColumn {
        position: usize,
        #[source]
        source: tokio_postgres::Error,
        table: &'static str,
    },

    #[error(
        "column {position} of a row read from table '{table}' holds JSON that does not decode: {source}"
    )]
    MalformedJson {
        position: usize,
        #[source]
        source: serde_json::Error,
        table: &'static str,
    },

    #[error(
        "the {statement:?} statement on table '{table}' writes a number outside its column's precision: {source}"
    )]
    NumericValueOutOfRange {
        #[source]
        source: DatabaseError,
        statement: StatementKind,
        table: &'static str,
    },

    #[error("a record of table '{table}' whose relations were requested no longer exists")]
    ParentVanished { table: &'static str },

    #[error(
        "the {statement:?} statement on table '{table}' cannot be serialized with concurrent transactions: {source}"
    )]
    SerializationFailure {
        #[source]
        source: DatabaseError,
        statement: StatementKind,
        table: &'static str,
    },

    #[error(
        "the {statement:?} statement on table '{table}' violates a unique constraint: {source}"
    )]
    UniqueViolation {
        #[source]
        source: DatabaseError,
        statement: StatementKind,
        table: &'static str,
    },

    #[error(
        "column {position} of a row read from table '{table}' stores '{stored}', which is not a variant of '{enum_type}'"
    )]
    UnknownVariant {
        enum_type: &'static str,
        position: usize,
        stored: String,
        table: &'static str,
    },
}

use tokio_postgres::error::SqlState;

use margaret_database::database_error::DatabaseError;

use crate::active_record_error::ActiveRecordError;
use crate::statement_kind::StatementKind;

fn state_of(source: &DatabaseError) -> Option<&SqlState> {
    match source {
        DatabaseError::StatementExecution(error) => error.code(),
        DatabaseError::Begin { .. }
        | DatabaseError::Commit(_)
        | DatabaseError::MalformedMaxConnections(_)
        | DatabaseError::MalformedUrl(_)
        | DatabaseError::PoolBuild(_)
        | DatabaseError::Rollback(_)
        | DatabaseError::StatementPreparation(_)
        | DatabaseError::Unavailable(_) => None,
    }
}

pub(crate) fn statement_error(
    statement: StatementKind,
    table: &'static str,
) -> impl FnOnce(DatabaseError) -> ActiveRecordError {
    move |source| match state_of(&source) {
        Some(state) if *state == SqlState::CHECK_VIOLATION => ActiveRecordError::CheckViolation {
            source,
            statement,
            table,
        },
        Some(state) if *state == SqlState::NUMERIC_VALUE_OUT_OF_RANGE => {
            ActiveRecordError::NumericValueOutOfRange {
                source,
                statement,
                table,
            }
        }
        Some(state) if *state == SqlState::UNIQUE_VIOLATION => ActiveRecordError::UniqueViolation {
            source,
            statement,
            table,
        },
        Some(state) if *state == SqlState::FOREIGN_KEY_VIOLATION => {
            ActiveRecordError::ForeignKeyViolation {
                source,
                statement,
                table,
            }
        }
        Some(state) if *state == SqlState::T_R_DEADLOCK_DETECTED => ActiveRecordError::Deadlock {
            source,
            statement,
            table,
        },
        Some(state) if *state == SqlState::T_R_SERIALIZATION_FAILURE => {
            ActiveRecordError::SerializationFailure {
                source,
                statement,
                table,
            }
        }
        Some(_) | None => ActiveRecordError::Database {
            source,
            statement,
            table,
        },
    }
}

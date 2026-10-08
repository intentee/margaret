use margaret_database::executor::Executor;
use margaret_sql::conflict_action::ConflictAction;
use margaret_sql::insert_source::InsertSource;
use margaret_sql::returning::Returning;

use crate::active_record_error::ActiveRecordError;
use crate::assignable::Assignable;
use crate::assigned::Assigned;
use crate::conflicting::Conflicting;
use crate::detached::Detached;
use crate::executed_affected::executed_affected;
use crate::field_set::FieldSet;
use crate::guarded_insert::GuardedInsert;
use crate::insert_statement::InsertStatement;
use crate::insertion::Insertion;
use crate::model::Model;
use crate::predicate::Predicate;
use crate::primary_key_columns::primary_key_columns;
use crate::record_values::record_values;
use crate::statement_kind::StatementKind;
use crate::unguarded::Unguarded;
use crate::upsert::Upsert;

pub struct Insert<'record, Modeled> {
    record: &'record Modeled,
}

impl<'record, Modeled: Model> Insert<'record, Modeled> {
    pub(crate) fn new(record: &'record Modeled) -> Self {
        Self { record }
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row cannot be inserted.
    pub async fn or_ignore<Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<Insertion, ActiveRecordError> {
        let statement = record_values(self.record).map(|values| {
            InsertStatement {
                conflict: ConflictAction::Ignore {
                    target: primary_key_columns::<Modeled>(),
                },
                returning: Returning::Nothing,
                source: InsertSource::Values,
            }
            .render(Modeled::TABLE, values)
        });

        executed_affected(statement, StatementKind::Insert, Modeled::TABLE, executor)
            .await
            .map(|inserted| match inserted {
                0 => Insertion::AlreadyPresent,
                _ => Insertion::Inserted,
            })
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row cannot be inserted.
    pub async fn run<Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<(), ActiveRecordError> {
        let statement = record_values(self.record).map(|values| {
            InsertStatement {
                conflict: ConflictAction::Raise,
                returning: Returning::Nothing,
                source: InsertSource::Values,
            }
            .render(Modeled::TABLE, values)
        });

        executed_affected(statement, StatementKind::Insert, Modeled::TABLE, executor)
            .await
            .map(|_inserted| ())
    }

    #[must_use]
    pub fn when(self, guard: Predicate<Detached>) -> GuardedInsert<'record, Modeled> {
        GuardedInsert::new(self.record, guard)
    }
}

impl<'record, Modeled: Assignable> Insert<'record, Modeled> {
    #[must_use]
    pub fn or_update(
        self,
        assign: impl FnOnce(Modeled::Columns<Conflicting>) -> Assigned<Modeled, Conflicting>,
    ) -> Upsert<'record, Modeled, Unguarded> {
        Upsert::new(self.record, assign(FieldSet::fields()), Unguarded)
    }
}

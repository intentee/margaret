use margaret_database::executor::Executor;
use margaret_sql::conflict_action::ConflictAction;
use margaret_sql::insert_source::InsertSource;
use margaret_sql::returning::Returning;

use crate::active_record_error::ActiveRecordError;
use crate::base_alias::BASE_ALIAS;
use crate::detached::Detached;
use crate::executed_affected::executed_affected;
use crate::guarded_insertion::GuardedInsertion;
use crate::insert_statement::InsertStatement;
use crate::model::Model;
use crate::predicate::Predicate;
use crate::record_values::record_values;
use crate::statement_kind::StatementKind;

pub struct GuardedInsert<'record, Modeled> {
    guard: Predicate<Detached>,
    record: &'record Modeled,
}

impl<'record, Modeled: Model> GuardedInsert<'record, Modeled> {
    pub(crate) fn new(record: &'record Modeled, guard: Predicate<Detached>) -> Self {
        Self { guard, record }
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row cannot be inserted.
    pub async fn run<Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<GuardedInsertion, ActiveRecordError> {
        let statement = self
            .guard
            .clause
            .condition(BASE_ALIAS, Modeled::TABLE)
            .and_then(|guard| {
                record_values(self.record).map(|values| {
                    InsertStatement {
                        conflict: ConflictAction::Raise,
                        returning: Returning::Nothing,
                        source: InsertSource::Guarded(guard),
                    }
                    .render(Modeled::TABLE, values)
                })
            });

        executed_affected(statement, StatementKind::Insert, Modeled::TABLE, executor)
            .await
            .map(|inserted| match inserted {
                0 => GuardedInsertion::Refused,
                _ => GuardedInsertion::Inserted,
            })
    }
}

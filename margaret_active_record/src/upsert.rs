use margaret_database::executor::Executor;
use margaret_sql::conflict_action::ConflictAction;
use margaret_sql::insert_source::InsertSource;
use margaret_sql::returning::Returning;

use crate::active_record_error::ActiveRecordError;
use crate::assignable::Assignable;
use crate::assigned::Assigned;
use crate::conflicting::Conflicting;
use crate::executed_affected::executed_affected;
use crate::field_set::FieldSet;
use crate::insert_statement::InsertStatement;
use crate::predicate::Predicate;
use crate::primary_key_columns::primary_key_columns;
use crate::record_values::record_values;
use crate::row_guard::RowGuard;
use crate::statement_kind::StatementKind;
use crate::unguarded::Unguarded;
use crate::upsertion::Upsertion;

fn upsertion(written: u64) -> Upsertion {
    match written {
        0 => Upsertion::Kept,
        _ => Upsertion::Written,
    }
}

pub struct Upsert<'record, Modeled, Guard> {
    assigned: Assigned<Modeled, Conflicting>,
    guard: Guard,
    record: &'record Modeled,
}

impl<'record, Modeled: Assignable, Guard> Upsert<'record, Modeled, Guard> {
    pub(crate) fn new(
        record: &'record Modeled,
        assigned: Assigned<Modeled, Conflicting>,
        guard: Guard,
    ) -> Self {
        Self {
            assigned,
            guard,
            record,
        }
    }
}

impl<'record, Modeled: Assignable> Upsert<'record, Modeled, Unguarded> {
    #[must_use]
    pub fn when(
        self,
        guard: impl FnOnce(Modeled::Conditions) -> Predicate<Modeled>,
    ) -> Upsert<'record, Modeled, Predicate<Modeled>> {
        Upsert::new(self.record, self.assigned, guard(FieldSet::fields()))
    }
}

impl<Modeled: Assignable, Guard: RowGuard> Upsert<'_, Modeled, Guard> {
    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row cannot be written.
    pub async fn run<Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<Upsertion, ActiveRecordError> {
        let Self {
            assigned,
            guard,
            record,
        } = self;
        let statement = assigned
            .assignments(Modeled::TABLE)
            .and_then(|assignments| {
                guard.conflict_filter(Modeled::TABLE).and_then(|filter| {
                    record_values(record).map(|values| {
                        InsertStatement {
                            conflict: ConflictAction::Update {
                                assignments,
                                filter,
                                target: primary_key_columns::<Modeled>(),
                            },
                            returning: Returning::Nothing,
                            source: InsertSource::Values,
                        }
                        .render(Modeled::TABLE, values)
                    })
                })
            });

        executed_affected(statement, StatementKind::Insert, Modeled::TABLE, executor)
            .await
            .map(upsertion)
    }
}

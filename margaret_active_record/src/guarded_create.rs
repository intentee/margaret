use margaret_database::executor::Executor;
use margaret_sql::conflict_action::ConflictAction;
use margaret_sql::insert_source::InsertSource;
use margaret_sql::returning::Returning;

use crate::active_record_error::ActiveRecordError;
use crate::base_alias::BASE_ALIAS;
use crate::creatable::Creatable;
use crate::creation::Creation;
use crate::detached::Detached;
use crate::draft_values::draft_values;
use crate::executed_optional_row::executed_optional_row;
use crate::insert_statement::InsertStatement;
use crate::predicate::Predicate;
use crate::read_record::read_record;
use crate::record_columns::record_columns;
use crate::statement_kind::StatementKind;

pub struct GuardedCreate<Modeled: Creatable> {
    draft: Modeled::Draft,
    guard: Predicate<Detached>,
}

impl<Modeled: Creatable> GuardedCreate<Modeled> {
    pub(crate) fn new(draft: Modeled::Draft, guard: Predicate<Detached>) -> Self {
        Self { draft, guard }
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row cannot be created.
    pub async fn run<Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<Creation<Modeled>, ActiveRecordError> {
        let statement = self
            .guard
            .clause
            .condition(BASE_ALIAS, Modeled::TABLE)
            .and_then(|guard| {
                draft_values(&self.draft, Modeled::TABLE).map(|values| {
                    InsertStatement {
                        conflict: ConflictAction::Raise,
                        returning: Returning::Columns(record_columns(Modeled::TABLE, BASE_ALIAS)),
                        source: InsertSource::Guarded(guard),
                    }
                    .render(Modeled::TABLE, values)
                })
            });

        executed_optional_row(statement, StatementKind::Insert, Modeled::TABLE, executor)
            .await
            .and_then(|row| match row {
                Some(row) => read_record(&row).map(Creation::Created),
                None => Ok(Creation::Refused),
            })
    }
}

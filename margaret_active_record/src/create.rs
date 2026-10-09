use margaret_database::executor::Executor;
use margaret_sql::conflict_action::ConflictAction;
use margaret_sql::insert_source::InsertSource;
use margaret_sql::returning::Returning;

use crate::active_record_error::ActiveRecordError;
use crate::base_alias::BASE_ALIAS;
use crate::creatable::Creatable;
use crate::detached::Detached;
use crate::draft_values::draft_values;
use crate::executed_row::executed_row;
use crate::guarded_create::GuardedCreate;
use crate::insert_statement::InsertStatement;
use crate::predicate::Predicate;
use crate::read_record::read_record;
use crate::record_columns::record_columns;
use crate::statement_kind::StatementKind;

pub struct Create<Modeled: Creatable> {
    draft: Modeled::Draft,
}

impl<Modeled: Creatable> Create<Modeled> {
    pub(crate) fn new(draft: Modeled::Draft) -> Self {
        Self { draft }
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row cannot be created.
    pub async fn run<Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<Modeled, ActiveRecordError> {
        let statement = draft_values(&self.draft, Modeled::TABLE).map(|values| {
            InsertStatement {
                conflict: ConflictAction::Raise,
                returning: Returning::Columns(record_columns(Modeled::TABLE, BASE_ALIAS)),
                source: InsertSource::Values,
            }
            .render(Modeled::TABLE, values)
        });

        executed_row(statement, StatementKind::Insert, Modeled::TABLE, executor)
            .await
            .and_then(|row| read_record(&row))
    }

    #[must_use]
    pub fn when(self, guard: Predicate<Detached>) -> GuardedCreate<Modeled> {
        GuardedCreate::new(self.draft, guard)
    }
}

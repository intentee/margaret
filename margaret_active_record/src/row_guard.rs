use margaret_model::table::Table;
use margaret_sql::conflict_filter::ConflictFilter;

use crate::active_record_error::ActiveRecordError;
use crate::base_alias::BASE_ALIAS;
use crate::clause::Clause;
use crate::predicate::Predicate;

pub trait RowGuard {
    fn conflict_filter(self, table: &'static Table) -> Result<ConflictFilter, ActiveRecordError>;

    fn guarded(self, clause: Clause) -> Clause;
}

impl<Row> RowGuard for Predicate<Row> {
    fn conflict_filter(self, table: &'static Table) -> Result<ConflictFilter, ActiveRecordError> {
        self.clause
            .condition(BASE_ALIAS, table)
            .map(ConflictFilter::When)
    }

    fn guarded(self, clause: Clause) -> Clause {
        clause.and(self.clause)
    }
}

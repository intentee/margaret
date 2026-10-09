use margaret_model::table::Table;
use margaret_sql::conflict_filter::ConflictFilter;

use crate::active_record_error::ActiveRecordError;
use crate::clause::Clause;
use crate::row_guard::RowGuard;

pub struct Unguarded;

impl RowGuard for Unguarded {
    fn conflict_filter(self, _: &'static Table) -> Result<ConflictFilter, ActiveRecordError> {
        Ok(ConflictFilter::Always)
    }

    fn guarded(self, clause: Clause) -> Clause {
        clause
    }
}

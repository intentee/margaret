use margaret_model::table::Table;
use margaret_sql::select_filter::SelectFilter;

use crate::active_record_error::ActiveRecordError;
use crate::base_alias::BASE_ALIAS;
use crate::selection::Selection;

pub(crate) fn select_filter(
    selection: Selection,
    table: &'static Table,
) -> Result<SelectFilter, ActiveRecordError> {
    match selection {
        Selection::Everything => Ok(SelectFilter::Everything),
        Selection::Matching(clause) => clause
            .condition(BASE_ALIAS, table)
            .map(SelectFilter::Matching),
    }
}

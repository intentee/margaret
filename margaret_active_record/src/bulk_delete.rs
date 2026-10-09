use margaret_database::executor::Executor;
use margaret_model::table::Table;
use margaret_sql::delete::Delete;
use margaret_sql::render_delete::render_delete;
use margaret_sql::returning::Returning;

use crate::active_record_error::ActiveRecordError;
use crate::base_alias::BASE_ALIAS;
use crate::clause::Clause;
use crate::executed_affected::executed_affected;
use crate::statement_kind::StatementKind;
use crate::table_source::table_source;

pub(crate) async fn bulk_delete<Executing: Executor>(
    clause: Clause,
    table: &'static Table,
    executor: &Executing,
) -> Result<u64, ActiveRecordError> {
    let statement = clause.condition(BASE_ALIAS, table).map(|condition| {
        render_delete(&Delete {
            condition,
            returning: Returning::Nothing,
            target: table_source(table, BASE_ALIAS),
        })
    });

    executed_affected(statement, StatementKind::Delete, table, executor).await
}

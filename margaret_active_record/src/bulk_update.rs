use margaret_database::executor::Executor;
use margaret_model::table::Table;
use margaret_sql::render_update::render_update;
use margaret_sql::returning::Returning;
use margaret_sql::update::Update;

use crate::active_record_error::ActiveRecordError;
use crate::assigned::Assigned;
use crate::assigning::Assigning;
use crate::base_alias::BASE_ALIAS;
use crate::clause::Clause;
use crate::executed_affected::executed_affected;
use crate::statement_kind::StatementKind;
use crate::table_source::table_source;

pub(crate) async fn bulk_update<Modeled, Executing: Executor>(
    clause: Clause,
    assigned: Assigned<Modeled, Assigning>,
    table: &'static Table,
    executor: &Executing,
) -> Result<u64, ActiveRecordError> {
    let statement = assigned.assignments(table).and_then(|assignments| {
        clause.condition(BASE_ALIAS, table).map(|condition| {
            render_update(&Update {
                assignments,
                condition,
                returning: Returning::Nothing,
                target: table_source(table, BASE_ALIAS),
            })
        })
    });

    executed_affected(statement, StatementKind::Update, table, executor).await
}

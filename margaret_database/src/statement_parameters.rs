use tokio_postgres::types::ToSql;

use margaret_sql::sql_parameter::SqlParameter;
use margaret_sql::statement::Statement;

pub(crate) fn statement_parameters(
    Statement { parameters, .. }: &Statement,
) -> Vec<&(dyn ToSql + Sync)> {
    parameters
        .iter()
        .map(|SqlParameter { value }| value.as_ref() as &(dyn ToSql + Sync))
        .collect()
}

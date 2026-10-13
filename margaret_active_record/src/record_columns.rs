use margaret_model::table::Table;
use margaret_sql::expression::Expression;
use margaret_sql::table_alias::TableAlias;

pub(crate) fn record_columns(table: &'static Table, alias: TableAlias) -> Vec<Expression> {
    table
        .columns
        .iter()
        .map(|column| Expression::Column {
            alias,
            column: column.name,
        })
        .collect()
}

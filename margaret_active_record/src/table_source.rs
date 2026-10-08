use margaret_model::table::Table;
use margaret_sql::table_alias::TableAlias;
use margaret_sql::table_source::TableSource;

pub(crate) fn table_source(table: &'static Table, alias: TableAlias) -> TableSource {
    TableSource {
        alias,
        namespace: table.namespace,
        table: table.name,
    }
}

use margaret_sql::table_alias::TableAlias;
use margaret_sql::table_source::TableSource;
use margaret_sql_identifier::table_namespace::TableNamespace;

pub const PROBES: TableSource = TableSource {
    alias: TableAlias { position: 0 },
    namespace: TableNamespace::Application,
    table: "probes",
};

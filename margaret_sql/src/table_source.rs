use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::table_alias::TableAlias;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableSource {
    pub alias: TableAlias,
    pub namespace: TableNamespace,
    pub table: &'static str,
}

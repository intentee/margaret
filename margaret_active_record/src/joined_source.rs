use margaret_sql::table_alias::TableAlias;

use crate::join_context::JoinContext;

#[derive(Clone, Copy)]
pub struct JoinedSource {
    pub(crate) alias: TableAlias,
    pub(crate) context: JoinContext,
}

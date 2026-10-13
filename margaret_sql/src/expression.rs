use crate::sql_parameter::SqlParameter;
use crate::table_alias::TableAlias;

#[derive(Clone)]
pub enum Expression {
    Column {
        alias: TableAlias,
        column: &'static str,
    },
    Excluded {
        column: &'static str,
    },
    Greatest {
        first: Box<Expression>,
        second: Box<Expression>,
    },
    Parameter(SqlParameter),
    SubqueryColumn {
        alias: TableAlias,
        position: usize,
    },
    UnnestColumn {
        alias: TableAlias,
        position: usize,
    },
}

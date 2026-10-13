use margaret_model::column_default::ColumnDefault;
use margaret_model::column_type::ColumnType;

use crate::resolved_check::ResolvedCheck;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedColumn {
    pub checks: Vec<ResolvedCheck>,
    pub column_type: ColumnType,
    pub default: ColumnDefault,
    pub name: String,
    pub nullable: bool,
}

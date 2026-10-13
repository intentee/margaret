use crate::column_check::ColumnCheck;
use crate::column_default::ColumnDefault;
use crate::column_type::ColumnType;

#[derive(Debug, Eq, PartialEq)]
pub struct Column {
    pub checks: &'static [ColumnCheck],
    pub column_type: ColumnType,
    pub default: ColumnDefault,
    pub name: &'static str,
    pub nullable: bool,
}

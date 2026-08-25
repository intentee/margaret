use crate::column_check::ColumnCheck;
use crate::column_default::ColumnDefault;
use crate::column_type::ColumnType;

pub struct Column {
    pub checks: Vec<ColumnCheck>,
    pub column_type: ColumnType,
    pub default: ColumnDefault,
    pub name: String,
    pub nullable: bool,
}

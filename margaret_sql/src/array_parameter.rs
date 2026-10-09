use margaret_model::column_type::ColumnType;

use crate::sql_parameter::SqlParameter;

#[derive(Clone)]
pub struct ArrayParameter {
    pub element_type: ColumnType,
    pub values: SqlParameter,
}

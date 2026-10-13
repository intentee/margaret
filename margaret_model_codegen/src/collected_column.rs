use margaret_model::column_default::ColumnDefault;
use margaret_model::column_type::ColumnType;

use crate::field_value::FieldValue;
use crate::resolved_check::ResolvedCheck;

pub(crate) struct CollectedColumn {
    pub(crate) checks: Vec<ResolvedCheck>,
    pub(crate) column_type: ColumnType,
    pub(crate) default: ColumnDefault,
    pub(crate) name: String,
    pub(crate) value: FieldValue,
}

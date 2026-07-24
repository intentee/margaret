use margaret_model::column_type::ColumnType;

use crate::column_id::ColumnId;

pub(crate) struct ForeignKeyTargetColumn {
    pub(crate) column_type: ColumnType,
    pub(crate) id: ColumnId,
    pub(crate) name: String,
}

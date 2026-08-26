use margaret_model::column_type::ColumnType;

pub(crate) struct ForeignKeyTargetColumn {
    pub(crate) column_type: ColumnType,
    pub(crate) name: String,
}

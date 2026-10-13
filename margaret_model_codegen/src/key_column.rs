use margaret_model::column_type::ColumnType;

#[derive(Clone)]
pub(crate) struct KeyColumn {
    pub(crate) column_type: ColumnType,
    pub(crate) name: String,
}

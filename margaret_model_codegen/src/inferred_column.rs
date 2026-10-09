use margaret_model::column_type::ColumnType;

#[derive(Debug)]
pub struct InferredColumn {
    pub column_type: ColumnType,
    pub nullable: bool,
}

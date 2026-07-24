use crate::column_index::ColumnIndex;
use crate::inferred_column::InferredColumn;

#[derive(Debug)]
pub struct ResolvedColumn {
    pub index: ColumnIndex,
    pub inferred: InferredColumn,
    pub name: String,
    pub position: usize,
    pub primary_key: bool,
    pub unique: bool,
}

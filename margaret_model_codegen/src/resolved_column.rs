use margaret_model::column_check::ColumnCheck;

use crate::inferred_column::InferredColumn;

#[derive(Debug)]
pub struct ResolvedColumn {
    pub checks: Vec<ColumnCheck>,
    pub inferred: InferredColumn,
    pub name: String,
    pub primary_key: bool,
}

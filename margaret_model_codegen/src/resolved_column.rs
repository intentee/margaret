use margaret_model::column_check::ColumnCheck;

use crate::index_membership::IndexMembership;
use crate::inferred_column::InferredColumn;

#[derive(Debug)]
pub struct ResolvedColumn {
    pub checks: Vec<ColumnCheck>,
    pub indexes: Vec<IndexMembership>,
    pub inferred: InferredColumn,
    pub name: String,
    pub position: usize,
    pub primary_key: bool,
    pub unique: bool,
}

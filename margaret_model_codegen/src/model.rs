use crate::resolved_column::ResolvedColumn;
use crate::resolved_foreign_key::ResolvedForeignKey;
use crate::resolved_index::ResolvedIndex;

#[derive(Debug)]
pub struct Model {
    pub columns: Vec<ResolvedColumn>,
    pub foreign_keys: Vec<ResolvedForeignKey>,
    pub indexes: Vec<ResolvedIndex>,
    pub table: String,
}

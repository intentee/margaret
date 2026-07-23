use crate::inferred_column::InferredColumn;

#[derive(Debug)]
pub struct ResolvedColumn {
    pub index: bool,
    pub inferred: InferredColumn,
    pub name: String,
    pub primary_key: bool,
    pub unique: bool,
}

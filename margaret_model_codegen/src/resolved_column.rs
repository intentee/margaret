use crate::inferred_column::InferredColumn;

pub(crate) struct ResolvedColumn {
    pub(crate) inferred: InferredColumn,
    pub(crate) name: String,
    pub(crate) primary_key: bool,
}

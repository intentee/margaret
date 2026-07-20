use crate::resolved_column::ResolvedColumn;

pub(crate) struct Model {
    pub(crate) columns: Vec<ResolvedColumn>,
    pub(crate) table: String,
}

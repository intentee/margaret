use crate::resolved_column::ResolvedColumn;

pub(crate) struct ScalarColumn {
    pub(crate) column: ResolvedColumn,
    pub(crate) unique: bool,
}

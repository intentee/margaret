use crate::resolved_column::ResolvedColumn;
use crate::resolved_foreign_key::ResolvedForeignKey;

pub(crate) struct Model {
    pub(crate) columns: Vec<ResolvedColumn>,
    pub(crate) foreign_keys: Vec<ResolvedForeignKey>,
    pub(crate) table: String,
}

use crate::resolved_column::ResolvedColumn;
use crate::resolved_foreign_key::ResolvedForeignKey;
use crate::resolved_index::ResolvedIndex;

pub(crate) struct Model {
    pub(crate) columns: Vec<ResolvedColumn>,
    pub(crate) foreign_keys: Vec<ResolvedForeignKey>,
    pub(crate) indexes: Vec<ResolvedIndex>,
    pub(crate) table: String,
}

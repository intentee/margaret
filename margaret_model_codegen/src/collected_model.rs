use std::collections::HashSet;

use crate::deferred_foreign_key::DeferredForeignKey;
use crate::resolved_column::ResolvedColumn;

pub(crate) struct CollectedModel {
    pub(crate) deferred_foreign_keys: Vec<DeferredForeignKey>,
    pub(crate) model: String,
    pub(crate) scalar_columns: Vec<ResolvedColumn>,
    pub(crate) seen_columns: HashSet<String>,
    pub(crate) table: String,
}

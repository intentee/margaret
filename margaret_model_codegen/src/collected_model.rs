use std::collections::HashSet;

use crate::deferred_foreign_key::DeferredForeignKey;
use crate::deferred_model_foreign_key::DeferredModelForeignKey;
use crate::resolved_column::ResolvedColumn;

pub(crate) struct CollectedModel {
    pub(crate) deferred_foreign_keys: Vec<DeferredForeignKey>,
    pub(crate) deferred_model_foreign_keys: Vec<DeferredModelForeignKey>,
    pub(crate) model: String,
    pub(crate) primary_key: Vec<String>,
    pub(crate) scalar_columns: Vec<ResolvedColumn>,
    pub(crate) seen_columns: HashSet<String>,
    pub(crate) table: String,
}

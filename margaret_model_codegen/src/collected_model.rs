use std::collections::HashSet;

use crate::deferred_foreign_key::DeferredForeignKey;
use crate::deferred_model_foreign_key::DeferredModelForeignKey;
use crate::model_index_arguments::ModelIndexArguments;
use crate::model_unique_arguments::ModelUniqueArguments;
use crate::resolved_column::ResolvedColumn;
use crate::resolved_index::ResolvedIndex;
use crate::resolved_unique_constraint::ResolvedUniqueConstraint;

pub(crate) struct CollectedModel {
    pub(crate) deferred_foreign_keys: Vec<DeferredForeignKey>,
    pub(crate) deferred_model_foreign_keys: Vec<DeferredModelForeignKey>,
    pub(crate) deferred_model_indexes: Vec<ModelIndexArguments>,
    pub(crate) deferred_model_unique_constraints: Vec<ModelUniqueArguments>,
    pub(crate) indexes: Vec<ResolvedIndex>,
    pub(crate) model: String,
    pub(crate) primary_key: Vec<String>,
    pub(crate) scalar_columns: Vec<ResolvedColumn>,
    pub(crate) seen_columns: HashSet<String>,
    pub(crate) table: String,
    pub(crate) unique_constraints: Vec<ResolvedUniqueConstraint>,
}

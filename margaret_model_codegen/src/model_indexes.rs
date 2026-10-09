use std::iter::once;

use crate::index_kind::IndexKind;
use crate::model_index::ModelIndex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelIndexes {
    pub primary_key: ModelIndex,
    pub secondary: Vec<ModelIndex>,
}

impl ModelIndexes {
    pub fn all(&self) -> impl Iterator<Item = &ModelIndex> {
        once(&self.primary_key).chain(self.secondary.iter())
    }

    pub fn of_kind(&self, kind: IndexKind) -> impl Iterator<Item = &ModelIndex> {
        self.all().filter(move |index| index.kind == kind)
    }
}

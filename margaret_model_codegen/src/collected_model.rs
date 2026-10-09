use margaret_attributes::canonical_path::CanonicalPath;

use crate::collected_field::CollectedField;
use crate::declared_index::DeclaredIndex;
use crate::declared_relation::DeclaredRelation;
use crate::primary_key_part::PrimaryKeyPart;

pub(crate) struct CollectedModel {
    pub(crate) fields: Vec<CollectedField>,
    pub(crate) indexes: Vec<DeclaredIndex>,
    pub(crate) module: String,
    pub(crate) path: CanonicalPath,
    pub(crate) primary_key: Vec<PrimaryKeyPart>,
    pub(crate) relations: Vec<DeclaredRelation>,
    pub(crate) table: String,
    pub(crate) uniques: Vec<Vec<String>>,
}

impl CollectedModel {
    pub(crate) fn primary_key_fields(&self) -> Vec<String> {
        self.primary_key
            .iter()
            .map(|part| part.field.clone())
            .collect()
    }
}

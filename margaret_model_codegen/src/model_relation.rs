use margaret_attributes::canonical_path::CanonicalPath;

use crate::model_field::ModelField;
use crate::model_index::ModelIndex;
use crate::relation_kind::RelationKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelRelation {
    pub covering_index: ModelIndex,
    pub key: ModelField,
    pub kind: RelationKind,
    pub name: String,
    pub related: CanonicalPath,
}

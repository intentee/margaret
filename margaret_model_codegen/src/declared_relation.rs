use margaret_attributes::canonical_path::CanonicalPath;

use crate::relation_kind::RelationKind;

#[derive(Clone)]
pub(crate) struct DeclaredRelation {
    pub(crate) key_field: String,
    pub(crate) kind: RelationKind,
    pub(crate) name: String,
    pub(crate) related: CanonicalPath,
}

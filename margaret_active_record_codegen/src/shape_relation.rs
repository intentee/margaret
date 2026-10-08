use margaret_attributes::canonical_path::CanonicalPath;

use crate::loaded_wrapper::LoadedWrapper;
use crate::shape_relation_kind::ShapeRelationKind;

pub(crate) struct ShapeRelation<'model> {
    pub(crate) field: String,
    pub(crate) kind: ShapeRelationKind<'model>,
    pub(crate) loaded: CanonicalPath,
    pub(crate) relation: String,
    pub(crate) wrapper: LoadedWrapper,
}

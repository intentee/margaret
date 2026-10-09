use margaret_attributes::canonical_path::CanonicalPath;
use margaret_model_codegen::model::Model;

use crate::shape_relation::ShapeRelation;

pub struct ShapeDeclaration<'model> {
    pub(crate) base: String,
    pub model: &'model Model,
    pub(crate) module: String,
    pub path: CanonicalPath,
    pub(crate) relations: Vec<ShapeRelation<'model>>,
}

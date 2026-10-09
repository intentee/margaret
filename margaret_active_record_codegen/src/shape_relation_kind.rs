use margaret_model_codegen::model_field::ModelField;
use margaret_model_codegen::model_relation::ModelRelation;

pub(crate) enum ShapeRelationKind<'model> {
    BelongsTo {
        key: &'model ModelField,
    },
    HasMany {
        limit: usize,
        relation: &'model ModelRelation,
    },
    HasOne {
        relation: &'model ModelRelation,
    },
}

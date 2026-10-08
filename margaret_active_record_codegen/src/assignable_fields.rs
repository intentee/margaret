use margaret_model_codegen::model::Model;
use margaret_model_codegen::model_field::ModelField;

pub(crate) fn assignable_fields(model: &Model) -> Vec<&ModelField> {
    model
        .fields
        .iter()
        .filter(|field| !model.indexes.primary_key.fields.contains(field))
        .collect()
}

use margaret_model_codegen::model_field::ModelField;

pub(crate) enum NodeOrdering<'model> {
    Partial,
    Total(Vec<&'model ModelField>),
}

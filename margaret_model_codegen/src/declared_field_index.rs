use margaret_attribute_arguments::attribute_args::AttributeArgs;

use crate::explicit_index_name::explicit_index_name;
use crate::field_index::FieldIndex;
use crate::model_codegen_error::ModelCodegenError;

pub(crate) fn declared_field_index(
    arguments: &AttributeArgs,
    model: &str,
    field: &str,
) -> Result<FieldIndex, ModelCodegenError> {
    arguments.interpret(|reader| {
        if reader.take_path_array("fields")?.is_some() {
            return Err(ModelCodegenError::FieldIndexCannotDeclareFields {
                field: field.to_string(),
                model: model.to_string(),
            });
        }

        match reader.take_string("name")? {
            None => Ok(FieldIndex::Derived),
            Some(name) => Ok(FieldIndex::Explicit(explicit_index_name(name, model)?)),
        }
    })
}

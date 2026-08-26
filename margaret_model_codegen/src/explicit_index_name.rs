use margaret_attributes::is_snake_case_identifier::is_snake_case_identifier;
use margaret_schema_identifier_naming::validate_identifier_length::validate_identifier_length;

use crate::model_codegen_error::ModelCodegenError;

pub(crate) fn explicit_index_name(name: String, model: &str) -> Result<String, ModelCodegenError> {
    if !is_snake_case_identifier(&name) {
        return Err(ModelCodegenError::InvalidIndexName {
            index: name,
            model: model.to_string(),
        });
    }

    if let Err(source) = validate_identifier_length(&name) {
        return Err(ModelCodegenError::ExplicitIndexNameTooLong {
            index: name,
            model: model.to_string(),
            source,
        });
    }

    Ok(name)
}

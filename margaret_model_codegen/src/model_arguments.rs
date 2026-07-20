use margaret_attributes::attribute_args::AttributeArgs;

use crate::model_codegen_error::ModelCodegenError;

pub(crate) struct ModelArguments {
    pub(crate) table: String,
}

impl ModelArguments {
    pub(crate) fn parse(arguments: &AttributeArgs, model: &str) -> Result<Self, ModelCodegenError> {
        let table = arguments
            .string("table")?
            .ok_or_else(|| ModelCodegenError::MissingTable {
                model: model.to_string(),
            })?;

        Ok(Self { table })
    }
}

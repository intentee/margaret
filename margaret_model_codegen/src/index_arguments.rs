use margaret_attribute_arguments::attribute_args::AttributeArgs;

use crate::model_codegen_error::ModelCodegenError;

pub(crate) struct IndexArguments {
    pub(crate) name: Option<String>,
}

impl IndexArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        model: &str,
        field: &str,
    ) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            if reader.take_path_array("columns")?.is_some() {
                return Err(ModelCodegenError::FieldIndexCannotDeclareColumns {
                    field: field.to_string(),
                    model: model.to_string(),
                });
            }

            let name = reader.take_string("name")?;

            Ok(Self { name })
        })
    }
}

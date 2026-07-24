use margaret_attributes::attribute_args::AttributeArgs;

use crate::model_codegen_error::ModelCodegenError;

pub(crate) struct IndexArguments {
    pub(crate) name: Option<String>,
}

impl IndexArguments {
    pub(crate) fn parse(arguments: &AttributeArgs) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let name = reader.take_string("name")?;

            Ok(Self { name })
        })
    }
}

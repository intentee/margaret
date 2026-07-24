use margaret_attributes::attribute_args::AttributeArgs;

use crate::model_codegen_error::ModelCodegenError;

pub(crate) struct ColumnArguments {
    pub(crate) name: Option<String>,
    pub(crate) primary_key: bool,
    pub(crate) unique: bool,
}

impl ColumnArguments {
    pub(crate) fn parse(arguments: &AttributeArgs) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let name = reader.take_string("name")?;
            let primary_key = reader.take_flag("primary_key");
            let unique = reader.take_flag("unique");

            Ok(Self {
                name,
                primary_key,
                unique,
            })
        })
    }
}

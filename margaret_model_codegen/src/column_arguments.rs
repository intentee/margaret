use margaret_attributes::attribute_args::AttributeArgs;

use crate::model_codegen_error::ModelCodegenError;

pub(crate) struct ColumnArguments {
    pub(crate) name: Option<String>,
    pub(crate) primary_key: bool,
    pub(crate) unique: bool,
}

impl ColumnArguments {
    pub(crate) fn parse(arguments: &AttributeArgs) -> Result<Self, ModelCodegenError> {
        let name = arguments.string("name")?;
        let primary_key = arguments.has_positional_flag("primary_key");
        let unique = arguments.has_positional_flag("unique");

        Ok(Self {
            name,
            primary_key,
            unique,
        })
    }
}

use margaret_attributes::attribute_args::AttributeArgs;

use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;

pub struct ConsoleArgumentArguments {
    pub from: String,
}

impl ConsoleArgumentArguments {
    pub fn parse(
        arguments: &AttributeArgs,
        owner: &str,
        position: usize,
    ) -> Result<Self, ConsoleArgumentCodegenError> {
        let from =
            arguments
                .string("from")?
                .ok_or_else(|| ConsoleArgumentCodegenError::MissingFrom {
                    owner: owner.to_string(),
                    parameter: position.to_string(),
                })?;

        Ok(Self { from })
    }
}

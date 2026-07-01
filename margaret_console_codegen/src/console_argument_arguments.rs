use margaret_attributes::attribute_args::AttributeArgs;

use crate::console_codegen_error::ConsoleCodegenError;

pub(crate) struct ConsoleArgumentArguments {
    pub(crate) from: String,
}

impl ConsoleArgumentArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        command: &str,
        position: usize,
    ) -> Result<Self, ConsoleCodegenError> {
        let from = arguments.string("from")?.ok_or_else(|| {
            ConsoleCodegenError::ConsoleArgumentMissingFrom {
                command: command.to_string(),
                parameter: position.to_string(),
            }
        })?;

        Ok(Self { from })
    }
}

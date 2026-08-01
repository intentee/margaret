use margaret_attribute_arguments::attribute_args::AttributeArgs;

use crate::console_codegen_error::ConsoleCodegenError;

pub(crate) struct ConsoleCommandArguments {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}

impl ConsoleCommandArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        command: &str,
    ) -> Result<Self, ConsoleCodegenError> {
        arguments.interpret(|reader| {
            let name = reader.take_string("name")?.ok_or_else(|| {
                ConsoleCodegenError::MissingCommandName {
                    command: command.to_string(),
                }
            })?;
            let description = reader.take_string("description")?;

            Ok(Self { name, description })
        })
    }
}

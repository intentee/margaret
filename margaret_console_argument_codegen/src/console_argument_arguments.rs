use margaret_attributes::attribute_args::AttributeArgs;

use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
use crate::console_argument_form::ConsoleArgumentForm;

pub struct ConsoleArgumentArguments {
    pub form: ConsoleArgumentForm,
}

impl ConsoleArgumentArguments {
    pub fn parse(
        arguments: &AttributeArgs,
        owner: &str,
        parameter: &str,
    ) -> Result<Self, ConsoleArgumentCodegenError> {
        arguments.expect_only(&["from"], &["positional"])?;

        let named = arguments.string("from")?;
        let positional = arguments.has_positional_flag("positional");

        let form = match (named, positional) {
            (Some(key), false) => ConsoleArgumentForm::Named { key },
            (None, true) => ConsoleArgumentForm::Positional,
            (Some(_), true) => {
                return Err(ConsoleArgumentCodegenError::NamedAndPositional {
                    owner: owner.to_string(),
                    parameter: parameter.to_string(),
                });
            }
            (None, false) => {
                return Err(ConsoleArgumentCodegenError::NeitherNamedNorPositional {
                    owner: owner.to_string(),
                    parameter: parameter.to_string(),
                });
            }
        };

        Ok(Self { form })
    }
}

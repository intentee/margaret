use margaret_attributes::attribute_args::AttributeArgs;

use crate::argument_relation::ArgumentRelation;
use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
use crate::console_argument_form::ConsoleArgumentForm;

fn build_relations(
    required_if: Option<String>,
    equals: Option<String>,
    owner: &str,
    parameter: &str,
) -> Result<Vec<ArgumentRelation>, ConsoleArgumentCodegenError> {
    match (required_if, equals) {
        (None, None) => Ok(Vec::new()),
        (Some(argument), Some(value)) => {
            Ok(vec![ArgumentRelation::RequiredIfEq { argument, value }])
        }
        (Some(_), None) | (None, Some(_)) => {
            Err(ConsoleArgumentCodegenError::IncompleteRequiredIf {
                owner: owner.to_string(),
                parameter: parameter.to_string(),
            })
        }
    }
}

pub struct ConsoleArgumentArguments {
    pub form: ConsoleArgumentForm,
    pub relations: Vec<ArgumentRelation>,
}

impl ConsoleArgumentArguments {
    pub fn parse(
        arguments: &AttributeArgs,
        owner: &str,
        parameter: &str,
    ) -> Result<Self, ConsoleArgumentCodegenError> {
        arguments.interpret(|reader| {
            let named = reader.take_string("from")?;
            let positional = reader.take_flag("positional");
            let required_if = reader.take_string("required_if")?;
            let equals = reader.take_string("equals")?;

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
            let relations = build_relations(required_if, equals, owner, parameter)?;

            Ok(Self { form, relations })
        })
    }
}

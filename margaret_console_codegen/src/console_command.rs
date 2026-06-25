use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_method::IndexedMethod;
use proc_macro2::Ident;
use quote::format_ident;
use syn::FnArg;
use syn::Type;

use crate::console_argument::ConsoleArgument;
use crate::console_argument_selector;
use crate::console_codegen_error::ConsoleCodegenError;
use crate::optional_parameter::OptionalParameter;

pub(crate) struct ConsoleCommand {
    pub(crate) accessor: Ident,
    pub(crate) arguments: Vec<ConsoleArgument>,
    pub(crate) description: Option<String>,
    pub(crate) name: String,
}

pub(crate) fn console_commands(
    index: &AttributeIndex,
) -> Result<Vec<ConsoleCommand>, ConsoleCodegenError> {
    let command_selector = AttributeSelector::parse("console_command").expect("a valid selector");
    let argument_selector = console_argument_selector();
    let mut commands = Vec::new();

    for matched in index.select(&command_selector) {
        let item = match matched.holder() {
            AttributeHolder::Item(item) if item.kind().is_struct() => item,
            holder => {
                return Err(ConsoleCodegenError::ConsoleCommandNotOnStruct {
                    target: holder.target_path(),
                });
            }
        };

        let attributes = matched.args()?;
        let command = item.canonical_path().to_string();
        let name = attributes
            .string("name")?
            .ok_or(ConsoleCodegenError::MissingCommandName {
                command: command.clone(),
            })?;
        let description = attributes.string("description")?;
        let accessor = format_ident!("{}", item.canonical_path().field_name());
        let arguments =
            command_arguments(index, item.canonical_path(), &argument_selector, &command)?;

        commands.push(ConsoleCommand {
            accessor,
            arguments,
            description,
            name,
        });
    }

    Ok(commands)
}

fn command_arguments(
    index: &AttributeIndex,
    command_path: &CanonicalPath,
    argument_selector: &AttributeSelector,
    command: &str,
) -> Result<Vec<ConsoleArgument>, ConsoleCodegenError> {
    let runner = command_runner(index, command_path).ok_or_else(|| {
        ConsoleCodegenError::MissingCommandRunner {
            command: command.to_string(),
        }
    })?;
    let mut arguments = Vec::new();
    let mut positional_index = 0;

    for (position, input) in runner.signature().inputs.iter().enumerate() {
        let FnArg::Typed(pattern_type) = input else {
            continue;
        };
        let Some(attribute) = pattern_type
            .attrs
            .iter()
            .find(|attribute| argument_selector.matches(attribute.path()))
        else {
            return Err(ConsoleCodegenError::UnmarkedRunnerParameter {
                command: command.to_string(),
                parameter: position.to_string(),
            });
        };

        let name = AttributeArgs::from_attribute(attribute)?.string("name")?;

        let argument = match (name, is_bool(&pattern_type.ty)) {
            (Some(name), true) => ConsoleArgument::Flag { name },
            (Some(name), false) => named_value(name, &pattern_type.ty),
            (None, true) => {
                return Err(ConsoleCodegenError::NamelessFlag {
                    command: command.to_string(),
                    parameter: position.to_string(),
                });
            }
            (None, false) => {
                let positional = positional_value(positional_index, &pattern_type.ty);
                positional_index += 1;

                positional
            }
        };

        arguments.push(argument);
    }

    Ok(arguments)
}

fn named_value(name: String, declared: &Type) -> ConsoleArgument {
    let OptionalParameter {
        required,
        value_type,
    } = OptionalParameter::from_type(declared);

    ConsoleArgument::Named {
        name,
        required,
        value_type,
    }
}

fn positional_value(positional_index: usize, declared: &Type) -> ConsoleArgument {
    let OptionalParameter {
        required,
        value_type,
    } = OptionalParameter::from_type(declared);

    ConsoleArgument::Positional {
        id: positional_index.to_string(),
        required,
        value_type,
    }
}

fn is_bool(declared: &Type) -> bool {
    matches!(declared, Type::Path(type_path) if type_path.path.is_ident("bool"))
}

fn command_runner<'index>(
    index: &'index AttributeIndex,
    command_path: &CanonicalPath,
) -> Option<&'index IndexedMethod> {
    let runner_selector = AttributeSelector::parse("runner").expect("a valid selector");

    index
        .select(&runner_selector)
        .into_iter()
        .find_map(|matched| match matched.holder() {
            AttributeHolder::Method(method) if method.self_type_path() == command_path => {
                Some(method)
            }
            _ => None,
        })
}

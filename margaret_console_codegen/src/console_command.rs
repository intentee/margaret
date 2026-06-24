use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::item_kind::ItemKind;
use proc_macro2::Ident;
use quote::format_ident;
use syn::FnArg;

use crate::console_argument::ConsoleArgument;
use crate::console_argument_selector;
use crate::console_codegen_error::ConsoleCodegenError;

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
            AttributeHolder::Item(item) if item.kind() == ItemKind::Struct => item,
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
    let constructor = command_constructor(index, command_path, command)?;
    let mut arguments = Vec::new();

    for (position, input) in constructor.signature().inputs.iter().enumerate() {
        let FnArg::Typed(pattern_type) = input else {
            continue;
        };
        let Some(attribute) = pattern_type
            .attrs
            .iter()
            .find(|attribute| argument_selector.matches(attribute.path()))
        else {
            continue;
        };

        let argument = AttributeArgs::from_attribute(attribute)?;
        let cli_name = argument.string("name")?.ok_or_else(|| {
            ConsoleCodegenError::MissingConsoleArgumentName {
                command: command.to_string(),
                parameter: position.to_string(),
            }
        })?;
        let required = argument.boolean("required")?.ok_or_else(|| {
            ConsoleCodegenError::MissingConsoleArgumentRequired {
                command: command.to_string(),
                parameter: position.to_string(),
            }
        })?;

        if !required {
            return Err(ConsoleCodegenError::ConsoleArgumentOptionalUnsupported {
                command: command.to_string(),
                parameter: position.to_string(),
            });
        }

        arguments.push(ConsoleArgument {
            cli_name,
            value_type: (*pattern_type.ty).clone(),
        });
    }

    Ok(arguments)
}

fn command_constructor<'index>(
    index: &'index AttributeIndex,
    command_path: &CanonicalPath,
    command: &str,
) -> Result<&'index IndexedMethod, ConsoleCodegenError> {
    let constructor_selector = AttributeSelector::parse("constructor").expect("a valid selector");

    index
        .select(&constructor_selector)
        .into_iter()
        .find_map(|matched| match matched.holder() {
            AttributeHolder::Method(method) if method.self_type_path() == command_path => {
                Some(method)
            }
            _ => None,
        })
        .ok_or_else(|| ConsoleCodegenError::MissingCommandConstructor {
            command: command.to_string(),
        })
}

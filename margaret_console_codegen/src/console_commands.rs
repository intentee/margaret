use std::collections::BTreeMap;

use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_console_argument_codegen::process_arguments::process_arguments;
use margaret_console_argument_codegen::required_argument_style::RequiredArgumentStyle;
use margaret_injection_codegen::is_cancellation_token::is_cancellation_token;
use margaret_injection_codegen::parameters::parameters;
use margaret_injection_codegen::process_method::process_method;

use crate::console_codegen_error::ConsoleCodegenError;
use crate::console_command::ConsoleCommand;
use crate::console_command_arguments::ConsoleCommandArguments;

fn runner_takes_token(index: &AttributeIndex, item: &IndexedItem, runner: &IndexedMethod) -> bool {
    parameters(runner.signature())
        .iter()
        .any(|view| is_cancellation_token(index, item, view.declared))
}

fn selector(name: &str) -> AttributeSelector {
    AttributeSelector::from_marker(name)
}

pub(crate) fn console_commands(
    index: &AttributeIndex,
) -> Result<Vec<ConsoleCommand>, ConsoleCodegenError> {
    let mut commands: BTreeMap<String, ConsoleCommand> = BTreeMap::new();

    for matched in index.select(&selector("console_command")) {
        let item = matched.item();

        if !item.kind().is_struct() {
            return Err(ConsoleCodegenError::ConsoleCommandNotOnStruct {
                target: item.canonical_path().to_string(),
            });
        }

        let command = item.canonical_path().to_string();
        let ConsoleCommandArguments { name, description } =
            ConsoleCommandArguments::parse(matched.args()?, &command)?;
        let accessor = format_ident!("{}", index.field_name(item.canonical_path()));
        let runner = process_method(item)?;
        let arguments = process_arguments(
            index,
            item,
            runner,
            &command,
            RequiredArgumentStyle::Positional,
        )?;

        let existing = commands.insert(
            name.clone(),
            ConsoleCommand {
                accessor,
                arguments,
                command_path: command.clone(),
                description,
                name: name.clone(),
                takes_token: runner_takes_token(index, item, runner),
            },
        );

        if let Some(existing) = existing {
            return Err(ConsoleCodegenError::DuplicateCommandName {
                command,
                existing_command: existing.command_path,
                name,
            });
        }
    }

    Ok(commands.into_values().collect())
}

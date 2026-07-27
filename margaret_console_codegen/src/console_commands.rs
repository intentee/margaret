use std::collections::BTreeMap;

use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_injection_codegen::is_cancellation_token::is_cancellation_token;
use margaret_injection_codegen::parameters::parameters;
use margaret_injection_codegen::process_method::process_method;
use margaret_injection_codegen::request_binding_marker::request_binding_marker;

use crate::console_codegen_error::ConsoleCodegenError;
use crate::console_command::ConsoleCommand;
use crate::console_command_arguments::ConsoleCommandArguments;

fn runner_takes_token(index: &AttributeIndex, item: &IndexedItem, runner: &IndexedMethod) -> bool {
    parameters(runner.signature())
        .iter()
        .any(|view| is_cancellation_token(index, item, view.declared))
}

fn validate_runner(
    index: &AttributeIndex,
    item: &IndexedItem,
    runner: &IndexedMethod,
    command: &str,
) -> Result<(), ConsoleCodegenError> {
    for view in parameters(runner.signature()) {
        if let Some(name) = request_binding_marker(view.attributes) {
            return Err(ConsoleCodegenError::ConsoleCommandRunnerRequestBinding {
                command: command.to_string(),
                parameter: view.holder.to_string(),
                marker: name.to_string(),
            });
        }

        if !is_cancellation_token(index, item, view.declared) {
            return Err(ConsoleCodegenError::ConsoleCommandRunnerArgument {
                command: command.to_string(),
                parameter: view.holder.to_string(),
            });
        }
    }

    Ok(())
}

fn selector(name: &str) -> AttributeSelector {
    AttributeSelector::from_marker(name)
}

pub(crate) fn console_commands(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
) -> Result<Vec<ConsoleCommand>, ConsoleCodegenError> {
    let mut commands: BTreeMap<String, ConsoleCommand> = BTreeMap::new();

    for matched in index.select(&selector("console_command")) {
        let item = matched.item();

        let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
            return Err(ConsoleCodegenError::ConsoleCommandNotOnStruct {
                target: item.canonical_path().to_string(),
            });
        };

        let command = item.canonical_path().to_string();
        let ConsoleCommandArguments { name, description } =
            ConsoleCommandArguments::parse(matched.args()?, &command)?;
        let accessor = format_ident!("{}", identifier.field());
        let runner = process_method(item)?;

        validate_runner(index, item, runner, &command)?;

        let arguments = bindings.console_arguments(item.canonical_path()).to_vec();

        if arguments
            .iter()
            .any(|argument| matches!(argument, ConsoleArgument::SpiffeHttpClient))
        {
            return Err(ConsoleCodegenError::ConsoleCommandInjectsSpiffeHttpClient { command });
        }

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

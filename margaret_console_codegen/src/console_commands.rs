use std::collections::BTreeMap;

use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_container::container_bindings::ContainerBindings;
use margaret_injection_codegen::process_method::process_method;
use margaret_injection_codegen::runner_signature::RunnerSignature;
use margaret_serve_input_codegen::has_spiffe_http_client::has_spiffe_http_client;

use crate::console_codegen_error::ConsoleCodegenError;
use crate::console_command::ConsoleCommand;
use crate::console_command_arguments::ConsoleCommandArguments;

pub(crate) fn console_commands(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
) -> Result<Vec<ConsoleCommand>, ConsoleCodegenError> {
    let mut commands: BTreeMap<String, ConsoleCommand> = BTreeMap::new();

    for matched in index.select_framework_attribute(FrameworkAttribute::ConsoleCommand) {
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

        let takes_token = match RunnerSignature::of(index, item, runner) {
            RunnerSignature::Accepted { takes_token } => takes_token,
            RunnerSignature::RejectedArgument { parameter } => {
                return Err(ConsoleCodegenError::ConsoleCommandRunnerArgument {
                    command,
                    parameter: parameter.to_string(),
                });
            }
            RunnerSignature::RejectedRequestBinding { marker, parameter } => {
                return Err(ConsoleCodegenError::ConsoleCommandRunnerRequestBinding {
                    command,
                    parameter: parameter.to_string(),
                    marker: marker.name().to_string(),
                });
            }
        };

        let serve_inputs = bindings
            .provider_serve_inputs(item.canonical_path())?
            .to_vec();

        if has_spiffe_http_client(serve_inputs.iter().map(|slotted| &slotted.input)) {
            return Err(ConsoleCodegenError::ConsoleCommandInjectsSpiffeHttpClient { command });
        }

        if serve_inputs
            .iter()
            .any(|slotted| slotted.input.reads_server_origins())
        {
            return Err(ConsoleCodegenError::ConsoleCommandDependsOnRouteUrl { command });
        }

        let existing = commands.insert(
            name.clone(),
            ConsoleCommand {
                accessor,
                serve_inputs,
                command_path: command.clone(),
                construction_root: item.canonical_path().clone(),
                description,
                is_async: runner.signature().asyncness.is_some(),
                name: name.clone(),
                takes_token,
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

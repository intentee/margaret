use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::http_server::HttpServer;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::console_codegen_error::ConsoleCodegenError;
use crate::console_command::ConsoleCommand;
use crate::console_commands::console_commands;
use crate::serve_command::ServeCommand;

pub struct ConsolePlan {
    pub(crate) commands: Vec<ConsoleCommand>,
    construction_roots: Vec<CanonicalPath>,
    pub(crate) has_models: bool,
    pub(crate) serve: Option<ServeCommand>,
}

impl ConsolePlan {
    /// # Errors
    ///
    /// Returns `ConsoleCodegenError` propagated from the work it performs.
    pub fn build(
        index: &AttributeIndex,
        bindings: &ContainerBindings,
        serves: bool,
        has_models: bool,
        http_servers: &[HttpServer],
        serve_inputs: &[ServeInput],
    ) -> Result<Self, ConsoleCodegenError> {
        let commands = console_commands(index, bindings)?;
        for generated_name in [(serves, "serve"), (has_models, "schema")]
            .into_iter()
            .filter_map(|(enabled, name)| enabled.then_some(name))
        {
            if let Some(command) = commands
                .iter()
                .find(|command| command.name == generated_name)
            {
                return Err(ConsoleCodegenError::GeneratedCommandNameCollision {
                    command: command.command_path.clone(),
                    name: generated_name,
                });
            }
        }
        let construction_roots = commands
            .iter()
            .map(|command| command.construction_root.clone())
            .collect();

        Ok(Self {
            commands,
            construction_roots,
            has_models,
            serve: serves
                .then(|| ServeCommand::build(http_servers, serve_inputs))
                .transpose()?,
        })
    }

    #[must_use]
    pub fn construction_roots(&self) -> &[CanonicalPath] {
        &self.construction_roots
    }
}

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;

use crate::console_codegen_error::ConsoleCodegenError;
use crate::console_command::ConsoleCommand;
use crate::console_commands::console_commands;
use crate::framework_command::FrameworkCommand;
use crate::serve_dispatch::ServeDispatch;

pub struct ConsolePlan {
    pub(crate) commands: Vec<ConsoleCommand>,
    construction_roots: Vec<CanonicalPath>,
    pub(crate) framework_commands: Vec<FrameworkCommand>,
}

impl ConsolePlan {
    /// # Errors
    ///
    /// Returns `ConsoleCodegenError` propagated from the work it performs.
    pub fn build(
        index: &AttributeIndex,
        bindings: &ContainerBindings,
        serve: &ServeDispatch,
        has_models: bool,
    ) -> Result<Self, ConsoleCodegenError> {
        let framework_commands = FrameworkCommand::generated(serve, has_models);
        let commands = console_commands(index, bindings, &framework_commands)?;
        let construction_roots = commands
            .iter()
            .map(|command| command.construction_root.clone())
            .collect();

        Ok(Self {
            commands,
            construction_roots,
            framework_commands,
        })
    }

    #[must_use]
    pub fn construction_roots(&self) -> &[CanonicalPath] {
        &self.construction_roots
    }
}

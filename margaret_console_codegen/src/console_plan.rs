use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;

use crate::console_codegen_error::ConsoleCodegenError;
use crate::console_command::ConsoleCommand;
use crate::console_commands::console_commands;

pub struct ConsolePlan {
    pub(crate) commands: Vec<ConsoleCommand>,
    construction_roots: Vec<CanonicalPath>,
}

impl ConsolePlan {
    /// # Errors
    ///
    /// Returns `ConsoleCodegenError` propagated from the work it performs.
    pub fn build(
        index: &AttributeIndex,
        bindings: &ContainerBindings,
    ) -> Result<Self, ConsoleCodegenError> {
        let commands = console_commands(index, bindings)?;
        let construction_roots = commands
            .iter()
            .map(|command| command.construction_root.clone())
            .collect();

        Ok(Self {
            commands,
            construction_roots,
        })
    }

    #[must_use]
    pub fn construction_roots(&self) -> &[CanonicalPath] {
        &self.construction_roots
    }
}

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use crate::console_argument::ConsoleArgument;
use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
use crate::serve_input_key::ServeInputKey;

pub struct ServeInputRegistry {
    pub arguments: Vec<ConsoleArgument>,
    pub slots: BTreeMap<ServeInputKey, usize>,
}

impl ServeInputRegistry {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            arguments: Vec::new(),
            slots: BTreeMap::new(),
        }
    }

    /// # Errors
    ///
    /// Returns `ConsoleArgumentCodegenError::ConflictingConsoleArgumentId`.
    pub fn register(
        &mut self,
        argument: ConsoleArgument,
    ) -> Result<usize, ConsoleArgumentCodegenError> {
        let key = argument.slot_key();
        let next_slot = self.arguments.len();

        match self.slots.entry(key) {
            Entry::Occupied(entry) => {
                let slot = *entry.get();
                let existing = &self.arguments[slot];
                let collides = matches!(argument, ConsoleArgument::Positional { .. })
                    || matches!(existing, ConsoleArgument::Positional { .. });

                if collides {
                    return Err(ConsoleArgumentCodegenError::ConflictingConsoleArgumentId {
                        name: argument.name().to_string(),
                    });
                }

                Ok(slot)
            }
            Entry::Vacant(entry) => {
                entry.insert(next_slot);
                self.arguments.push(argument);
                Ok(next_slot)
            }
        }
    }
}

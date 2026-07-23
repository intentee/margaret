use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::console_argument::ConsoleArgument;

#[derive(Debug)]
pub struct ConsoleArgumentRegistry {
    arguments: BTreeMap<CanonicalPath, BTreeMap<usize, ConsoleArgument>>,
}

impl ConsoleArgumentRegistry {
    #[must_use]
    pub(crate) fn new(
        arguments: BTreeMap<CanonicalPath, BTreeMap<usize, ConsoleArgument>>,
    ) -> Self {
        Self { arguments }
    }

    #[must_use]
    pub fn argument(&self, owner: &CanonicalPath, position: usize) -> Option<&ConsoleArgument> {
        self.arguments
            .get(owner)
            .and_then(|by_position| by_position.get(&position))
    }
}

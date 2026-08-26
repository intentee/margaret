use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::serve_input::ServeInput;

#[derive(Debug)]
pub struct DeclaredServeInputs {
    inputs: BTreeMap<CanonicalPath, BTreeMap<usize, ServeInput>>,
}

impl DeclaredServeInputs {
    #[must_use]
    pub(crate) fn new(inputs: BTreeMap<CanonicalPath, BTreeMap<usize, ServeInput>>) -> Self {
        Self { inputs }
    }

    #[must_use]
    pub fn input(&self, owner: &CanonicalPath, position: usize) -> Option<&ServeInput> {
        self.inputs
            .get(owner)
            .and_then(|by_position| by_position.get(&position))
    }
}

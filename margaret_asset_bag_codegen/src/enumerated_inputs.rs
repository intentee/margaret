use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::entrypoint_output::EntrypointOutput;

pub(crate) struct EnumeratedInputs {
    pub(crate) entrypoints: BTreeMap<String, EntrypointOutput>,
    pub(crate) static_inputs: BTreeSet<String>,
}

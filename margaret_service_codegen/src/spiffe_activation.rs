use std::collections::BTreeSet;

use margaret_serve_input_codegen::spiffe_client_kind::SpiffeClientKind;

#[derive(Clone)]
pub(crate) struct SpiffeActivation {
    pub(crate) server_active: bool,
    pub(crate) spiffe_clients: BTreeSet<SpiffeClientKind>,
}

impl SpiffeActivation {
    pub(crate) fn client_active(&self) -> bool {
        !self.spiffe_clients.is_empty()
    }
}

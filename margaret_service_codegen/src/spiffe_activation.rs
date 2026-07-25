#[derive(Clone, Copy)]
pub(crate) struct SpiffeActivation {
    pub(crate) client_active: bool,
    pub(crate) server_active: bool,
}

impl SpiffeActivation {
    pub(crate) fn svid_active(self) -> bool {
        self.client_active || self.server_active
    }
}

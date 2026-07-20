pub struct RequestBindingPolicy {
    pub allows_body_form_requests: bool,
    pub allows_forwarder: bool,
}

impl RequestBindingPolicy {
    #[must_use]
    pub fn full_request() -> Self {
        Self {
            allows_body_form_requests: true,
            allows_forwarder: true,
        }
    }

    #[must_use]
    pub fn handshake() -> Self {
        Self {
            allows_body_form_requests: false,
            allows_forwarder: false,
        }
    }
}

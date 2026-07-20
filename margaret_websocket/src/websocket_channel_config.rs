#[derive(Clone, Copy)]
pub struct WebSocketChannelConfig {
    pub internal_capacity: usize,
    pub outbound_capacity: usize,
}

impl WebSocketChannelConfig {
    #[must_use]
    pub fn recommended() -> Self {
        Self {
            internal_capacity: 16,
            outbound_capacity: 32,
        }
    }
}

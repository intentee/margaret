use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::websocket_client::web_socket_client::WebSocketClient;

#[singleton]
pub struct RelayClient {
    websocket_client: WebSocketClient,
}

impl RelayClient {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[spiffe_websocket_client] websocket_client: WebSocketClient,
    ) -> anyhow::Result<Self> {
        Ok(Self { websocket_client })
    }

    #[must_use]
    pub fn websocket_client(&self) -> &WebSocketClient {
        &self.websocket_client
    }
}

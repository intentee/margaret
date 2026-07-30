use tokio::sync::mpsc::Sender;

use crate::web_socket_driver::WebSocketDriver;

#[derive(Clone)]
pub struct WebSocketDriverSender {
    inner: Sender<WebSocketDriver>,
}

impl WebSocketDriverSender {
    pub(crate) fn new(inner: Sender<WebSocketDriver>) -> Self {
        Self { inner }
    }

    /// # Errors
    ///
    /// Returns `WebSocketDriver` propagated from the work it performs.
    pub async fn send(&self, driver: WebSocketDriver) -> Result<(), WebSocketDriver> {
        self.inner.send(driver).await.map_err(|error| error.0)
    }
}

use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use crate::web_socket_driver::WebSocketDriver;

#[derive(Clone)]
pub struct WebSocketDriverSender {
    inner: Sender<WebSocketDriver>,
}

impl WebSocketDriverSender {
    pub async fn send(&self, driver: WebSocketDriver) -> Result<(), WebSocketDriver> {
        self.inner.send(driver).await.map_err(|error| error.0)
    }
}

pub(crate) fn web_socket_driver_channel() -> (WebSocketDriverSender, Receiver<WebSocketDriver>) {
    let (sender, receiver) = mpsc::channel(1);

    (WebSocketDriverSender { inner: sender }, receiver)
}

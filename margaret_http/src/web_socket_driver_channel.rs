use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

use crate::web_socket_driver::WebSocketDriver;
use crate::web_socket_driver_sender::WebSocketDriverSender;

pub(crate) struct WebSocketDriverChannel {
    pub(crate) receiver: Receiver<WebSocketDriver>,
    pub(crate) sender: WebSocketDriverSender,
}

impl WebSocketDriverChannel {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = mpsc::channel(1);

        Self {
            receiver,
            sender: WebSocketDriverSender::new(sender),
        }
    }
}

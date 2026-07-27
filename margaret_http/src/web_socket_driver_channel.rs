use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

use crate::web_socket_driver::WebSocketDriver;
use crate::web_socket_driver_sender::WebSocketDriverSender;

pub(crate) fn web_socket_driver_channel() -> (WebSocketDriverSender, Receiver<WebSocketDriver>) {
    let (sender, receiver) = mpsc::channel(1);

    (WebSocketDriverSender::new(sender), receiver)
}

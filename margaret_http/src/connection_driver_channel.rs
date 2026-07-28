use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::unbounded_channel;

use crate::connection_driver::ConnectionDriver;
use crate::connection_driver_sender::ConnectionDriverSender;

pub(crate) struct ConnectionDriverChannel {
    pub receiver: UnboundedReceiver<ConnectionDriver>,
    pub sender: ConnectionDriverSender,
}

impl ConnectionDriverChannel {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = unbounded_channel();

        Self {
            receiver,
            sender: ConnectionDriverSender::new(sender),
        }
    }
}

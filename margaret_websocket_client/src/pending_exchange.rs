use tokio::sync::mpsc::Sender;

use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

use crate::exchange_termination::ExchangeTermination;

#[derive(Clone)]
pub(crate) struct PendingExchange {
    pub(crate) sender: Sender<ServerSentFrame>,
    pub(crate) termination: ExchangeTermination,
}

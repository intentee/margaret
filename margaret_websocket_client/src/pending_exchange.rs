use tokio::sync::mpsc::UnboundedSender;

use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

use crate::exchange_termination::ExchangeTermination;
use crate::granted_credit::GrantedCredit;

#[derive(Clone)]
pub(crate) struct PendingExchange {
    pub(crate) granted: GrantedCredit,
    pub(crate) sender: UnboundedSender<ServerSentFrame>,
    pub(crate) termination: ExchangeTermination,
}

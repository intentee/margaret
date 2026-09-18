use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc::Sender;

use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

pub(crate) type PendingResponse = Sender<ServerSentFrame>;

#[derive(Clone, Default)]
pub(crate) struct PendingResponses {
    senders: Arc<DashMap<RequestId, PendingResponse>>,
}

impl PendingResponses {
    pub(crate) fn clear(&self) {
        self.senders.clear();
    }

    pub(crate) fn forget(&self, id: &RequestId) {
        self.senders.remove(id);
    }

    pub(crate) fn peek(&self, id: &RequestId) -> Option<PendingResponse> {
        self.senders.get(id).map(|entry| entry.value().clone())
    }

    pub(crate) fn remember(&self, id: RequestId, sender: PendingResponse) {
        self.senders.insert(id, sender);
    }

    pub(crate) fn take(&self, id: &RequestId) -> Option<PendingResponse> {
        self.senders.remove(id).map(|(_, sender)| sender)
    }
}

use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

use crate::pending_responses::PendingResponses;

pub(crate) async fn route_server_frame(frame: ServerSentFrame, pending: &PendingResponses) {
    let id = frame.id().clone();
    let sender = if frame.is_final() {
        pending.take(&id)
    } else {
        pending.peek(&id)
    };

    if let Some(sender) = sender
        && sender.send(frame).await.is_err()
    {
        pending.forget(&id);
    }
}

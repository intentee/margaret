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

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use tokio::sync::mpsc;

    use margaret_websocket_envelope::request_id::RequestId;
    use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

    use super::route_server_frame;
    use crate::pending_responses::PendingResponses;

    fn response(id: RequestId) -> ServerSentFrame {
        ServerSentFrame::Response {
            id,
            is_done: false,
            method: "response_chunk".to_string(),
            payload: Value::Null,
        }
    }

    #[tokio::test]
    async fn forgets_an_exchange_whose_reader_is_gone() {
        let pending = PendingResponses::default();
        let id = RequestId::Number(1);
        let (sender, receiver) = mpsc::channel(1);

        pending.remember(id.clone(), sender);
        drop(receiver);

        route_server_frame(response(id.clone()), &pending).await;

        assert!(pending.peek(&id).is_none());
    }

    #[tokio::test]
    async fn drops_a_frame_that_belongs_to_no_exchange() {
        let pending = PendingResponses::default();

        route_server_frame(response(RequestId::Number(7)), &pending).await;

        assert!(pending.peek(&RequestId::Number(7)).is_none());
    }
}

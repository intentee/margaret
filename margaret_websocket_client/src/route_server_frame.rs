use tokio::sync::mpsc::error::TrySendError;

use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

use crate::exchange_interruption::ExchangeInterruption;
use crate::pending_responses::PendingResponses;

pub(crate) fn route_server_frame(frame: ServerSentFrame, pending: &PendingResponses) {
    let id = frame.id().clone();
    let exchange = if frame.is_final() {
        pending.take(&id)
    } else {
        pending.peek(&id)
    };

    let Some(exchange) = exchange else {
        return;
    };

    match exchange.sender.try_send(frame) {
        Ok(()) => {}
        Err(TrySendError::Closed(_)) => pending.forget(&id),
        Err(TrySendError::Full(_)) => {
            exchange
                .termination
                .interrupt(ExchangeInterruption::ResponseBacklogExceeded);
            pending.forget(&id);
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use tokio::sync::mpsc;

    use margaret_websocket_envelope::request_id::RequestId;
    use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

    use super::route_server_frame;
    use crate::exchange_outcome::ExchangeOutcome;
    use crate::exchange_termination::ExchangeTermination;
    use crate::pending_exchange::PendingExchange;
    use crate::pending_responses::PendingResponses;
    use crate::response_backlog_limit::RESPONSE_BACKLOG_LIMIT;

    fn response(id: RequestId) -> ServerSentFrame {
        ServerSentFrame::Response {
            id,
            is_done: false,
            method: "response_chunk".to_string(),
            payload: Value::Null,
        }
    }

    #[test]
    fn forgets_an_exchange_whose_reader_is_gone() {
        let pending = PendingResponses::default();
        let id = RequestId::Number(1);
        let (sender, receiver) = mpsc::channel(RESPONSE_BACKLOG_LIMIT);

        pending.remember(
            id.clone(),
            PendingExchange {
                sender,
                termination: ExchangeTermination::default(),
            },
        );
        drop(receiver);

        route_server_frame(response(id.clone()), &pending);

        assert!(pending.peek(&id).is_none());
    }

    #[test]
    fn drops_a_frame_that_belongs_to_no_exchange() {
        let pending = PendingResponses::default();

        let absent = RequestId::Number(7);

        route_server_frame(response(absent.clone()), &pending);

        assert!(pending.peek(&absent).is_none());
    }

    #[test]
    fn interrupts_an_exchange_whose_consumer_stopped_draining() {
        let pending = PendingResponses::default();
        let id = RequestId::Number(2);
        let (sender, _receiver) = mpsc::channel(RESPONSE_BACKLOG_LIMIT);
        let termination = ExchangeTermination::default();

        pending.remember(
            id.clone(),
            PendingExchange {
                sender,
                termination: termination.clone(),
            },
        );

        for _ in 0..=RESPONSE_BACKLOG_LIMIT {
            route_server_frame(response(id.clone()), &pending);
        }

        assert!(pending.peek(&id).is_none());
        assert!(matches!(
            termination.outcome(),
            ExchangeOutcome::Interrupted(interruption)
                if interruption == crate::exchange_interruption::ExchangeInterruption::ResponseBacklogExceeded
        ));
    }
}

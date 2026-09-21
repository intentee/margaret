use tokio::sync::mpsc::error::TrySendError;

use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

use crate::exchange_interruption::ExchangeInterruption;
use crate::exchange_registry::ExchangeRegistry;

pub(crate) fn route_server_frame(frame: ServerSentFrame, registry: &ExchangeRegistry) {
    let id = frame.id().clone();
    let exchange = if frame.is_final() {
        registry.take(&id)
    } else {
        registry.peek(&id)
    };

    let Some(exchange) = exchange else {
        return;
    };

    match exchange.sender.try_send(frame) {
        Ok(()) => {}
        Err(TrySendError::Closed(_)) => registry.forget(&id),
        Err(TrySendError::Full(_)) => {
            exchange
                .termination
                .interrupt(ExchangeInterruption::PeerExceededCredit);
            registry.forget(&id);
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
    use crate::exchange_interruption::ExchangeInterruption;
    use crate::exchange_outcome::ExchangeOutcome;
    use crate::exchange_queue_capacity::EXCHANGE_QUEUE_CAPACITY;
    use crate::exchange_registry::ExchangeRegistry;
    use crate::exchange_termination::ExchangeTermination;
    use crate::pending_exchange::PendingExchange;

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
        let registry = ExchangeRegistry::default();
        let id = RequestId::Number(1);
        let (sender, receiver) = mpsc::channel(EXCHANGE_QUEUE_CAPACITY);

        registry.register(
            &id,
            PendingExchange {
                sender,
                termination: ExchangeTermination::default(),
            },
        );
        drop(receiver);

        route_server_frame(response(id.clone()), &registry);

        assert!(registry.peek(&id).is_none());
    }

    #[test]
    fn drops_a_frame_that_belongs_to_no_exchange() {
        let registry = ExchangeRegistry::default();

        let absent = RequestId::Number(7);

        route_server_frame(response(absent.clone()), &registry);

        assert!(registry.peek(&absent).is_none());
    }

    #[test]
    fn interrupts_an_exchange_whose_peer_ran_past_its_credit() {
        let registry = ExchangeRegistry::default();
        let id = RequestId::Number(2);
        let (sender, _receiver) = mpsc::channel(EXCHANGE_QUEUE_CAPACITY);
        let termination = ExchangeTermination::default();

        registry.register(
            &id,
            PendingExchange {
                sender,
                termination: termination.clone(),
            },
        );

        for _ in 0..=EXCHANGE_QUEUE_CAPACITY {
            route_server_frame(response(id.clone()), &registry);
        }

        assert!(registry.peek(&id).is_none());
        assert!(matches!(
            termination.outcome(),
            ExchangeOutcome::Interrupted(interruption)
                if interruption == ExchangeInterruption::PeerExceededCredit
        ));
    }
}

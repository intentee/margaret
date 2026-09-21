use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

use crate::exchange_interruption::ExchangeInterruption;
use crate::exchange_registry::ExchangeRegistry;
use crate::granted_credit_outcome::GrantedCreditOutcome;

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

    if frame.is_metered() && exchange.granted.spend_one_frame() == GrantedCreditOutcome::Exhausted {
        exchange
            .termination
            .interrupt(ExchangeInterruption::PeerExceededCredit);
        registry.forget(&id);

        return;
    }

    if exchange.sender.send(frame).is_err() {
        registry.forget(&id);
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use tokio::sync::mpsc;

    use margaret_websocket_envelope::envelope_error::EnvelopeError;
    use margaret_websocket_envelope::envelope_error_code::EnvelopeErrorCode;
    use margaret_websocket_envelope::request_id::RequestId;
    use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

    use super::route_server_frame;
    use crate::exchange_interruption::ExchangeInterruption;
    use crate::exchange_outcome::ExchangeOutcome;
    use crate::exchange_registry::ExchangeRegistry;
    use crate::exchange_termination::ExchangeTermination;
    use crate::granted_credit::GrantedCredit;
    use crate::pending_exchange::PendingExchange;
    use crate::response_credit_window::RESPONSE_CREDIT_WINDOW;

    fn response(id: RequestId) -> ServerSentFrame {
        ServerSentFrame::Response {
            id,
            is_done: false,
            method: "response_chunk".to_string(),
            payload: Value::Null,
        }
    }

    fn rejection(id: RequestId) -> ServerSentFrame {
        ServerSentFrame::Error {
            error: EnvelopeError {
                code: EnvelopeErrorCode::InternalError,
                details: Value::Null,
                message: "the handler gave up".to_string(),
            },
            id,
        }
    }

    #[test]
    fn forgets_an_exchange_whose_reader_is_gone() {
        let registry = ExchangeRegistry::default();
        let id = RequestId::Number(1);
        let (sender, receiver) = mpsc::unbounded_channel();

        registry.register(
            &id,
            PendingExchange {
                granted: GrantedCredit::new(RESPONSE_CREDIT_WINDOW),
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
        let (sender, _receiver) = mpsc::unbounded_channel();
        let termination = ExchangeTermination::default();

        registry.register(
            &id,
            PendingExchange {
                granted: GrantedCredit::new(RESPONSE_CREDIT_WINDOW),
                sender,
                termination: termination.clone(),
            },
        );

        for _ in 0..=RESPONSE_CREDIT_WINDOW.frames() {
            route_server_frame(response(id.clone()), &registry);
        }

        assert!(registry.peek(&id).is_none());
        assert_eq!(
            termination.outcome(),
            ExchangeOutcome::Interrupted(ExchangeInterruption::PeerExceededCredit)
        );
    }

    #[test]
    fn carries_a_rejection_that_the_spent_window_cannot_pay_for() {
        let registry = ExchangeRegistry::default();
        let id = RequestId::Number(3);
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let termination = ExchangeTermination::default();

        registry.register(
            &id,
            PendingExchange {
                granted: GrantedCredit::new(RESPONSE_CREDIT_WINDOW),
                sender,
                termination: termination.clone(),
            },
        );

        for _ in 0..RESPONSE_CREDIT_WINDOW.frames() {
            route_server_frame(response(id.clone()), &registry);
        }

        route_server_frame(rejection(id.clone()), &registry);

        let delivered = std::iter::from_fn(|| receiver.try_recv().ok()).count();

        assert_eq!(delivered, RESPONSE_CREDIT_WINDOW.frames() + 1);
        assert_eq!(termination.outcome(), ExchangeOutcome::Completed);
    }
}

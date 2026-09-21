use std::sync::Arc;

use dashmap::DashMap;
use dashmap::mapref::entry::Entry;
use tokio_util::sync::CancellationToken;

use margaret_websocket_envelope::credit_grant::CreditGrant;
use margaret_websocket_envelope::request_id::RequestId;

use crate::credit_grant_outcome::CreditGrantOutcome;
use crate::exchange_admission::ExchangeAdmission;
use crate::served_exchange::ServedExchange;

#[derive(Clone, Default)]
pub(crate) struct ExchangeCredit {
    exchanges: Arc<DashMap<RequestId, ServedExchange>>,
}

impl ExchangeCredit {
    pub(crate) fn cancel(&self, id: &RequestId) {
        if let Some((_, exchange)) = self.exchanges.remove(id) {
            exchange.end();
        }
    }

    pub(crate) fn complete(&self, id: &RequestId, exchange: &ServedExchange) {
        drop(
            self.exchanges
                .remove_if(id, |_, open| open.is_same_exchange(exchange)),
        );
        exchange.end();
    }

    pub(crate) fn grant(&self, id: &RequestId, credit: CreditGrant) -> CreditGrantOutcome {
        let Some(exchange) = self.exchanges.get(id) else {
            return CreditGrantOutcome::ExchangeIsOver;
        };

        exchange.value().grant(credit)
    }

    pub(crate) fn open(
        &self,
        id: RequestId,
        window: CreditGrant,
        cancellation_token: CancellationToken,
    ) -> ExchangeAdmission {
        match self.exchanges.entry(id) {
            Entry::Occupied(_) => ExchangeAdmission::AlreadyOpen,
            Entry::Vacant(vacant) => {
                let exchange = ServedExchange::new(cancellation_token, window);

                vacant.insert(exchange.clone());

                ExchangeAdmission::Admitted(exchange)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use futures_util::FutureExt;
    use tokio_util::sync::CancellationToken;

    use margaret_websocket_envelope::credit_grant::CreditGrant;
    use margaret_websocket_envelope::request_id::RequestId;

    use super::CreditGrantOutcome;
    use super::ExchangeAdmission;
    use super::ExchangeCredit;
    use super::ServedExchange;
    use crate::exchange_flow_control::ExchangeFlowControl;
    use crate::web_socket_error::WebSocketError;

    const EXHAUSTED_WINDOW: CreditGrant = CreditGrant::from_frames(0);
    const ONE_FRAME: CreditGrant = CreditGrant::from_frames(1);
    const WHOLE_WINDOW: CreditGrant = CreditGrant::from_frames(u16::MAX);

    fn served(admission: ExchangeAdmission) -> Option<ServedExchange> {
        match admission {
            ExchangeAdmission::Admitted(exchange) => Some(exchange),
            ExchangeAdmission::AlreadyOpen => None,
        }
    }

    fn admitted(credit: &ExchangeCredit, id: RequestId, window: CreditGrant) -> ServedExchange {
        served(credit.open(id, window, CancellationToken::new()))
            .expect("an unused request id admits an exchange")
    }

    fn flow_control(exchange: &ServedExchange) -> ExchangeFlowControl {
        ExchangeFlowControl::Metered(exchange.credit.clone())
    }

    #[tokio::test]
    async fn resumes_an_exchange_once_more_credit_arrives() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);
        let exchange = admitted(&credit, id.clone(), EXHAUSTED_WINDOW);

        assert!(
            flow_control(&exchange)
                .spend_one_frame()
                .now_or_never()
                .is_none()
        );
        assert_eq!(credit.grant(&id, ONE_FRAME), CreditGrantOutcome::Granted);

        flow_control(&exchange)
            .spend_one_frame()
            .now_or_never()
            .expect("replenished credit is available at once")
            .expect("replenished credit is spendable");
    }

    #[tokio::test]
    async fn reports_a_frame_the_ended_exchange_can_no_longer_carry() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);
        let exchange = admitted(&credit, id.clone(), ONE_FRAME);

        credit.cancel(&id);

        let refused = flow_control(&exchange)
            .spend_one_frame()
            .await
            .expect_err("an exchange that ended spends nothing");

        assert!(matches!(
            refused,
            WebSocketError::ExchangeFinished { .. } if exchange.credit.is_closed()
        ));
    }

    #[tokio::test]
    async fn cancels_the_work_of_an_exchange_that_ends() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);
        let cancellation_token = CancellationToken::new();

        drop(served(credit.open(
            id.clone(),
            ONE_FRAME,
            cancellation_token.clone(),
        )));
        credit.cancel(&id);

        assert!(cancellation_token.is_cancelled());
    }

    #[tokio::test]
    async fn ends_an_exchange_that_is_already_over() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);

        credit.cancel(&id);

        assert_eq!(
            credit.grant(&id, ONE_FRAME),
            CreditGrantOutcome::ExchangeIsOver
        );
    }

    #[tokio::test]
    async fn ignores_credit_granted_to_an_exchange_that_is_over() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);

        assert_eq!(
            credit.grant(&id, ONE_FRAME),
            CreditGrantOutcome::ExchangeIsOver
        );

        let exchange = admitted(&credit, id, EXHAUSTED_WINDOW);

        assert!(
            flow_control(&exchange)
                .spend_one_frame()
                .now_or_never()
                .is_none()
        );
    }

    #[tokio::test]
    async fn refuses_a_grant_that_would_pass_the_credit_window() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);

        drop(admitted(&credit, id.clone(), WHOLE_WINDOW));

        assert_eq!(
            credit.grant(&id, ONE_FRAME),
            CreditGrantOutcome::WindowExceeded
        );
    }

    #[tokio::test]
    async fn releases_the_request_id_of_an_exchange_that_completes() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);
        let exchange = admitted(&credit, id.clone(), ONE_FRAME);

        credit.complete(&id, &exchange);

        assert!(exchange.credit.is_closed());
        assert!(served(credit.open(id, ONE_FRAME, CancellationToken::new())).is_some());
    }

    #[tokio::test]
    async fn keeps_the_exchange_that_took_a_cancelled_request_id() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);
        let cancelled = admitted(&credit, id.clone(), ONE_FRAME);

        credit.cancel(&id);

        let reopened = admitted(&credit, id.clone(), ONE_FRAME);

        credit.complete(&id, &cancelled);

        assert_eq!(credit.grant(&id, ONE_FRAME), CreditGrantOutcome::Granted);
        assert!(!reopened.credit.is_closed());
    }

    #[tokio::test]
    async fn refuses_a_second_exchange_that_reuses_an_open_request_id() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);

        drop(admitted(&credit, id.clone(), ONE_FRAME));

        assert!(served(credit.open(id, ONE_FRAME, CancellationToken::new())).is_none());
    }
}

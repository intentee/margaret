use std::sync::Arc;

use dashmap::DashMap;
use dashmap::mapref::entry::Entry;
use tokio::sync::Semaphore;

use margaret_websocket_envelope::request_id::RequestId;

use crate::exchange_admission::ExchangeAdmission;

#[derive(Clone, Default)]
pub(crate) struct ExchangeCredit {
    exchanges: Arc<DashMap<RequestId, Arc<Semaphore>>>,
}

impl ExchangeCredit {
    pub(crate) fn forget(&self, id: &RequestId) {
        self.exchanges.remove(id);
    }

    pub(crate) fn grant(&self, id: &RequestId, credit: usize) {
        if let Some(exchange) = self.exchanges.get(id) {
            exchange.value().add_permits(credit);
        }
    }

    pub(crate) fn open(&self, id: RequestId, credit: usize) -> ExchangeAdmission {
        match self.exchanges.entry(id) {
            Entry::Occupied(_) => ExchangeAdmission::AlreadyOpen,
            Entry::Vacant(vacant) => {
                let granted = Arc::new(Semaphore::new(credit));

                vacant.insert(granted.clone());

                ExchangeAdmission::Admitted(granted)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures_util::FutureExt;
    use tokio::sync::Semaphore;

    use margaret_websocket_envelope::request_id::RequestId;

    use super::ExchangeAdmission;
    use super::ExchangeCredit;
    use crate::exchange_flow_control::ExchangeFlowControl;
    use crate::web_socket_error::WebSocketError;

    const EXHAUSTED_WINDOW: usize = 0;
    const ONE_FRAME: usize = 1;

    fn metered(admission: ExchangeAdmission) -> Option<ExchangeFlowControl> {
        match admission {
            ExchangeAdmission::Admitted(granted) => Some(ExchangeFlowControl::Metered(granted)),
            ExchangeAdmission::AlreadyOpen => None,
        }
    }

    fn admitted(credit: &ExchangeCredit, id: RequestId, window: usize) -> ExchangeFlowControl {
        metered(credit.open(id, window)).expect("an unused request id admits an exchange")
    }

    #[tokio::test]
    async fn resumes_an_exchange_once_more_credit_arrives() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);
        let flow_control = admitted(&credit, id.clone(), EXHAUSTED_WINDOW);

        assert!(flow_control.spend_one_frame().now_or_never().is_none());

        credit.grant(&id, ONE_FRAME);

        flow_control
            .spend_one_frame()
            .now_or_never()
            .expect("replenished credit is available at once")
            .expect("replenished credit is spendable");
    }

    #[tokio::test]
    async fn reports_a_frame_the_forgotten_exchange_can_no_longer_carry() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);
        let granted = Arc::new(Semaphore::new(ONE_FRAME));

        credit.open(id.clone(), ONE_FRAME);
        credit.forget(&id);
        granted.close();

        let refused = ExchangeFlowControl::Metered(granted.clone())
            .spend_one_frame()
            .await
            .expect_err("a finished exchange spends nothing");

        assert!(matches!(
            refused,
            WebSocketError::ExchangeFinished { .. } if granted.is_closed()
        ));
    }

    #[tokio::test]
    async fn ignores_credit_granted_to_an_exchange_that_is_over() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);

        credit.grant(&id, ONE_FRAME);

        let flow_control = admitted(&credit, id, EXHAUSTED_WINDOW);

        assert!(flow_control.spend_one_frame().now_or_never().is_none());
    }

    #[tokio::test]
    async fn refuses_a_second_exchange_that_reuses_an_open_request_id() {
        let credit = ExchangeCredit::default();
        let id = RequestId::Number(1);

        drop(admitted(&credit, id.clone(), ONE_FRAME));

        assert!(metered(credit.open(id, ONE_FRAME)).is_none());
    }
}

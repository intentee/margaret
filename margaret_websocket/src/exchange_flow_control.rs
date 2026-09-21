use std::sync::Arc;

use tokio::sync::Semaphore;

use crate::exchange_state::ExchangeState;
use crate::web_socket_error::WebSocketError;

#[derive(Clone)]
pub(crate) enum ExchangeFlowControl {
    Metered(Arc<Semaphore>),
    Unmetered,
}

impl ExchangeFlowControl {
    pub(crate) fn exchange_state(&self) -> ExchangeState {
        match self {
            Self::Metered(credit) if credit.is_closed() => ExchangeState::Finished,
            Self::Metered(_) | Self::Unmetered => ExchangeState::Open,
        }
    }

    pub(crate) async fn spend_one_frame(&self) -> Result<(), WebSocketError> {
        match self {
            Self::Metered(credit) => credit
                .acquire()
                .await
                .map_err(|source| WebSocketError::ExchangeFinished { source })?
                .forget(),
            Self::Unmetered => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use futures_util::FutureExt;

    use super::Arc;
    use super::ExchangeFlowControl;
    use super::Semaphore;

    const GRANTED_FRAMES: usize = 2;

    #[tokio::test]
    async fn spends_only_the_frames_the_exchange_granted() {
        let flow_control = ExchangeFlowControl::Metered(Arc::new(Semaphore::new(GRANTED_FRAMES)));

        for _ in 0..GRANTED_FRAMES {
            flow_control
                .spend_one_frame()
                .await
                .expect("a granted frame is spendable");
        }

        assert!(flow_control.spend_one_frame().now_or_never().is_none());
    }

    #[tokio::test]
    async fn never_meters_what_a_notification_sends() {
        let flow_control = ExchangeFlowControl::Unmetered;

        for _ in 0..=GRANTED_FRAMES {
            flow_control
                .spend_one_frame()
                .now_or_never()
                .expect("a notification never waits on credit")
                .expect("a notification never runs out of credit");
        }
    }
}

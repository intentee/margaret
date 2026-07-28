use std::future::Future;
use std::mem::replace;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use bytes::Bytes;
use http_body_util::Full;
use tokio_util::sync::CancellationToken;

use crate::connection_driver::ConnectionDriver;
use crate::connection_driver_sender::ConnectionDriverSender;

type DispatchedResponse = Pin<Box<dyn Future<Output = http::Response<Full<Bytes>>> + Send>>;

enum CooperativeRequestState {
    Completed,
    Running(DispatchedResponse),
}

pub(crate) struct CooperativeRequest {
    cancellation_token: CancellationToken,
    driver_sender: ConnectionDriverSender,
    state: CooperativeRequestState,
}

impl CooperativeRequest {
    pub(crate) fn new<Dispatch>(
        dispatch: Dispatch,
        cancellation_token: CancellationToken,
        driver_sender: ConnectionDriverSender,
    ) -> Self
    where
        Dispatch: Future<Output = http::Response<Full<Bytes>>> + Send + 'static,
    {
        Self {
            cancellation_token,
            driver_sender,
            state: CooperativeRequestState::Running(Box::pin(dispatch)),
        }
    }
}

impl Future for CooperativeRequest {
    type Output = http::Response<Full<Bytes>>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let cooperative_request = self.get_mut();

        match &mut cooperative_request.state {
            CooperativeRequestState::Completed => Poll::Pending,
            CooperativeRequestState::Running(dispatch) => match dispatch.as_mut().poll(context) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(response) => {
                    cooperative_request.state = CooperativeRequestState::Completed;

                    Poll::Ready(response)
                }
            },
        }
    }
}

impl Drop for CooperativeRequest {
    fn drop(&mut self) {
        self.cancellation_token.cancel();

        let CooperativeRequestState::Running(dispatch) =
            replace(&mut self.state, CooperativeRequestState::Completed)
        else {
            return;
        };
        let continuation: ConnectionDriver = Box::pin(async move {
            drop(dispatch.await);
        });

        match self.driver_sender.send(continuation) {
            Ok(()) => {}
            Err(undriven_continuation) => drop(undriven_continuation),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::pending;
    use std::future::poll_fn;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use std::task::Poll;

    use bytes::Bytes;
    use http_body_util::Full;
    use tokio_util::sync::CancellationToken;

    use super::CooperativeRequest;
    use crate::connection_driver_channel::ConnectionDriverChannel;
    use crate::response::Response;

    fn ok_response() -> http::Response<Full<Bytes>> {
        Response::text(200, "ok").into_http()
    }

    #[tokio::test]
    async fn cancels_the_request_token_once_the_response_is_produced() {
        let ConnectionDriverChannel { receiver, sender } = ConnectionDriverChannel::new();
        let cancellation_token = CancellationToken::new();
        let cooperative_request =
            CooperativeRequest::new(async { ok_response() }, cancellation_token.clone(), sender);

        assert_eq!(cooperative_request.await.status().as_u16(), 200);
        assert!(cancellation_token.is_cancelled());
        drop(receiver);
    }

    #[tokio::test]
    async fn hands_the_remaining_work_to_the_connection_when_it_is_abandoned() {
        let ConnectionDriverChannel {
            mut receiver,
            sender,
        } = ConnectionDriverChannel::new();
        let cancellation_token = CancellationToken::new();
        let cleaned_up = Arc::new(AtomicBool::new(false));
        let cleaned_up_by_responder = Arc::clone(&cleaned_up);
        let observed_cancellation = cancellation_token.clone();

        drop(CooperativeRequest::new(
            async move {
                observed_cancellation.cancelled().await;
                cleaned_up_by_responder.store(true, Ordering::SeqCst);

                ok_response()
            },
            cancellation_token.clone(),
            sender,
        ));

        assert!(cancellation_token.is_cancelled());
        assert!(!cleaned_up.load(Ordering::SeqCst));

        receiver
            .recv()
            .await
            .expect("the abandoned request is handed to the connection")
            .await;

        assert!(cleaned_up.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn drops_the_continuation_when_the_connection_can_no_longer_drive_it() {
        let ConnectionDriverChannel { receiver, sender } = ConnectionDriverChannel::new();
        let cancellation_token = CancellationToken::new();

        drop(receiver);
        drop(CooperativeRequest::new(
            pending(),
            cancellation_token.clone(),
            sender,
        ));

        assert!(cancellation_token.is_cancelled());
    }

    #[tokio::test]
    async fn stays_pending_when_it_is_polled_again_after_completing() {
        let ConnectionDriverChannel { receiver, sender } = ConnectionDriverChannel::new();
        let mut cooperative_request =
            CooperativeRequest::new(async { ok_response() }, CancellationToken::new(), sender);

        assert!(
            poll_fn(|context| Poll::Ready(Pin::new(&mut cooperative_request).poll(context)))
                .await
                .is_ready()
        );
        assert!(
            poll_fn(|context| Poll::Ready(Pin::new(&mut cooperative_request).poll(context)))
                .await
                .is_pending()
        );

        drop(receiver);
    }
}

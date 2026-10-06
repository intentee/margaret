use std::mem;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;

use crate::stall_outcome::StallOutcome;
use crate::stall_wait::StallWait;

pub(crate) struct StallTimer {
    limit: Duration,
    wait: StallWait,
}

impl StallTimer {
    pub(crate) fn new(limit: Duration) -> Self {
        Self {
            limit,
            wait: StallWait::Idle,
        }
    }

    pub(crate) fn poll_progress<TOutput>(
        &mut self,
        context: &mut Context<'_>,
        outcome: Poll<TOutput>,
    ) -> Poll<StallOutcome<TOutput>> {
        match outcome {
            Poll::Ready(output) => {
                self.wait = StallWait::Idle;

                Poll::Ready(StallOutcome::Progressed(output))
            }
            Poll::Pending => {
                let mut stall = match mem::replace(&mut self.wait, StallWait::Idle) {
                    StallWait::Idle => Box::pin(tokio::time::sleep(self.limit)),
                    StallWait::Waiting { stall } => stall,
                };
                let elapsed = stall.as_mut().poll(context);

                self.wait = StallWait::Waiting { stall };

                elapsed.map(|()| StallOutcome::Stalled)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::poll_fn;
    use std::task::Poll;

    use super::StallTimer;
    use crate::client_body_timeout::CLIENT_BODY_TIMEOUT;
    use crate::stall_outcome::StallOutcome;

    async fn stalls_when(timer: &mut StallTimer, outcome: Poll<()>) -> bool {
        poll_fn(|context| {
            Poll::Ready(matches!(
                timer.poll_progress(context, outcome),
                Poll::Ready(StallOutcome::Stalled)
            ))
        })
        .await
    }

    #[tokio::test(start_paused = true)]
    async fn reports_a_stall_once_the_limit_passes_without_progress() {
        let mut timer = StallTimer::new(CLIENT_BODY_TIMEOUT);

        assert!(!stalls_when(&mut timer, Poll::Pending).await);
        tokio::time::advance(CLIENT_BODY_TIMEOUT).await;

        assert!(stalls_when(&mut timer, Poll::Pending).await);
    }

    #[tokio::test(start_paused = true)]
    async fn restarts_the_limit_after_progress() {
        let mut timer = StallTimer::new(CLIENT_BODY_TIMEOUT);

        assert!(!stalls_when(&mut timer, Poll::Pending).await);
        tokio::time::advance(CLIENT_BODY_TIMEOUT / 2).await;
        assert!(!stalls_when(&mut timer, Poll::Ready(())).await);
        assert!(!stalls_when(&mut timer, Poll::Pending).await);
        tokio::time::advance(CLIENT_BODY_TIMEOUT / 2).await;

        assert!(!stalls_when(&mut timer, Poll::Pending).await);
    }
}

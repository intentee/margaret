use tokio_util::sync::CancellationToken;

use margaret_console::command_outcome::CommandOutcome;

pub async fn run(cancellation_token: CancellationToken) -> CommandOutcome {
    cancellation_token.cancelled().await;

    CommandOutcome::Succeeded
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::pin;
    use std::task::Context;
    use std::task::Poll;
    use std::task::Waker;

    use tokio_util::sync::CancellationToken;

    use margaret_console::command_outcome::CommandOutcome;

    use super::run;

    #[test]
    fn runs_until_the_cancellation_token_is_cancelled() {
        let cancellation_token = CancellationToken::new();
        let mut identity = pin!(run(cancellation_token.clone()));
        let mut context = Context::from_waker(Waker::noop());

        assert_eq!(identity.as_mut().poll(&mut context), Poll::Pending);

        cancellation_token.cancel();

        assert_eq!(
            identity.as_mut().poll(&mut context),
            Poll::Ready(CommandOutcome::Succeeded)
        );
    }
}

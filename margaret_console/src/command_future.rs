use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use crate::command_outcome::CommandOutcome;

pub struct CommandFuture {
    inner: Pin<Box<dyn Future<Output = CommandOutcome>>>,
}

impl CommandFuture {
    pub fn new(inner: impl Future<Output = CommandOutcome> + 'static) -> Self {
        Self {
            inner: Box::pin(inner),
        }
    }
}

impl Future for CommandFuture {
    type Output = CommandOutcome;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<CommandOutcome> {
        self.inner.as_mut().poll(context)
    }
}

#[cfg(test)]
mod tests {
    use super::CommandFuture;
    use crate::command_outcome::CommandOutcome;

    #[tokio::test]
    async fn resolves_to_the_outcome_of_the_command_it_runs() {
        assert_eq!(
            CommandFuture::new(async { CommandOutcome::Failed }).await,
            CommandOutcome::Failed
        );
    }
}

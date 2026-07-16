use std::future::Future;
use std::io::Result as IoResult;

use tokio_util::sync::CancellationToken;

use margaret_console::command_outcome::CommandOutcome;
use margaret_console::report_failure::report_failure;

pub async fn dispatch_serve<Install, Serve, ServeFuture>(
    install: Install,
    serve: Serve,
) -> CommandOutcome
where
    Install: FnOnce() -> IoResult<CancellationToken>,
    Serve: FnOnce(CancellationToken) -> ServeFuture,
    ServeFuture: Future<Output = CommandOutcome>,
{
    match install() {
        Ok(cancellation_token) => serve(cancellation_token).await,
        Err(error) => report_failure(error),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Error;

    use tokio_util::sync::CancellationToken;

    use margaret_console::command_outcome::CommandOutcome;

    use super::dispatch_serve;

    async fn succeed(_cancellation_token: CancellationToken) -> CommandOutcome {
        CommandOutcome::Succeeded
    }

    #[tokio::test]
    async fn serves_with_the_installed_cancellation_token() {
        let outcome = dispatch_serve(|| Ok(CancellationToken::new()), succeed).await;

        assert_eq!(outcome, CommandOutcome::Succeeded);
    }

    #[tokio::test]
    async fn reports_failure_when_signal_installation_fails() {
        let outcome = dispatch_serve(
            || Err(Error::other("cannot install signal handlers")),
            succeed,
        )
        .await;

        assert_eq!(outcome, CommandOutcome::Failed);
    }
}

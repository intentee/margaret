use std::future::Future;
use std::io::Result;

use tokio_util::sync::CancellationToken;

use margaret_console::command_outcome::CommandOutcome;
use margaret_console::report_failure::report_failure;

use crate::shutdown_signals::ShutdownSignals;

async fn dispatch_installed<SignalFuture, Serve, ServeFuture>(
    signal: SignalFuture,
    serve: Serve,
) -> CommandOutcome
where
    SignalFuture: Future<Output = Result<()>>,
    Serve: FnOnce(CancellationToken) -> ServeFuture,
    ServeFuture: Future<Output = CommandOutcome>,
{
    let cancellation_token = CancellationToken::new();
    let serving = serve(cancellation_token.clone());
    tokio::pin!(serving);

    tokio::select! {
        outcome = &mut serving => outcome,
        signal = signal => {
            cancellation_token.cancel();
            let outcome = serving.await;

            match signal {
                Ok(()) => outcome,
                Err(error) => report_failure(error),
            }
        }
    }
}

pub async fn dispatch_serve<Install, Serve, ServeFuture>(
    install: Install,
    serve: Serve,
) -> CommandOutcome
where
    Install: FnOnce() -> Result<ShutdownSignals>,
    Serve: FnOnce(CancellationToken) -> ServeFuture,
    ServeFuture: Future<Output = CommandOutcome>,
{
    let signals = match install() {
        Ok(signals) => signals,
        Err(error) => return report_failure(error),
    };

    dispatch_installed(signals.wait(), serve).await
}

#[cfg(test)]
mod tests {
    use std::io::Error;

    use tokio_util::sync::CancellationToken;

    use margaret_console::command_outcome::CommandOutcome;

    use super::dispatch_installed;
    use super::dispatch_serve;
    use crate::install::install;
    use crate::shutdown_signals::ShutdownSignals;

    async fn succeed(_cancellation_token: CancellationToken) -> CommandOutcome {
        CommandOutcome::Succeeded
    }

    #[tokio::test]
    async fn serves_with_the_installed_cancellation_token() {
        let outcome = dispatch_serve(install, succeed).await;

        assert_eq!(outcome, CommandOutcome::Succeeded);
    }

    #[tokio::test]
    async fn reports_failure_when_signal_installation_fails() {
        let outcome = dispatch_serve(
            || Err::<ShutdownSignals, _>(Error::other("cannot install signal handlers")),
            succeed,
        )
        .await;

        assert_eq!(outcome, CommandOutcome::Failed);
    }

    async fn succeed_after_cancellation(cancellation_token: CancellationToken) -> CommandOutcome {
        cancellation_token.cancelled().await;

        CommandOutcome::Succeeded
    }

    #[tokio::test]
    async fn cancels_serving_and_preserves_its_outcome_after_a_shutdown_signal() {
        let outcome = dispatch_installed(async { Ok(()) }, succeed_after_cancellation).await;

        assert_eq!(outcome, CommandOutcome::Succeeded);
    }

    #[tokio::test]
    async fn reports_a_shutdown_signal_stream_failure_after_cancelling_serving() {
        let outcome = dispatch_installed(
            async { Err(Error::other("shutdown signal stream ended")) },
            succeed_after_cancellation,
        )
        .await;

        assert_eq!(outcome, CommandOutcome::Failed);
    }
}

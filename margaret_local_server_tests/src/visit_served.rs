use std::future::Future;
use std::process;

use margaret_process_tests::process_signal::ProcessSignal;
use margaret_process_tests::signal_process::signal_process;

use crate::poll_until::poll_until;

/// # Panics
///
/// Panics when the served process cannot be stopped.
pub async fn visit_served<TOutcome, TProbe>(
    serving: impl Future<Output = TOutcome>,
    ready: impl Fn() -> TProbe,
    visit: impl AsyncFnOnce(),
) -> TOutcome
where
    TProbe: Future<Output = bool>,
{
    let visiting = async {
        poll_until(ready).await;
        visit().await;

        assert!(signal_process(process::id(), ProcessSignal::Terminate).success());
    };

    let ((), outcome) = tokio::join!(visiting, serving);

    outcome
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    use tokio::signal::unix::SignalKind;
    use tokio::signal::unix::signal;

    use super::visit_served;

    #[tokio::test]
    async fn stops_the_served_process_after_the_visit_and_answers_its_outcome() {
        let mut terminations =
            signal(SignalKind::terminate()).expect("the termination signal is observed");
        let visited = AtomicBool::new(false);

        assert_eq!(
            visit_served(
                async {
                    terminations.recv().await;

                    "stopped"
                },
                || async { true },
                async || visited.store(true, Ordering::SeqCst),
            )
            .await,
            "stopped"
        );
        assert!(visited.load(Ordering::SeqCst));
    }
}

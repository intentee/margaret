use tokio::time::Instant;
use tokio::time::sleep_until;
use tokio_util::sync::CancellationToken;

use crate::deadline_wake::DeadlineWake;

pub async fn await_deadline(
    deadline: Instant,
    cancellation_token: &CancellationToken,
) -> DeadlineWake {
    tokio::select! {
        biased;
        () = cancellation_token.cancelled() => DeadlineWake::Cancelled,
        () = sleep_until(deadline) => DeadlineWake::Reached,
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::Instant;
    use tokio_util::sync::CancellationToken;

    use super::await_deadline;
    use crate::deadline_wake::DeadlineWake;

    const WAIT: Duration = Duration::from_mins(1);

    #[tokio::test(start_paused = true)]
    async fn wakes_when_the_deadline_is_reached() {
        let started_at = Instant::now();

        assert_eq!(
            await_deadline(started_at + WAIT, &CancellationToken::new()).await,
            DeadlineWake::Reached
        );
        assert_eq!(started_at.elapsed(), WAIT);
    }

    #[tokio::test(start_paused = true)]
    async fn wakes_immediately_for_a_deadline_already_passed() {
        let started_at = Instant::now();

        assert_eq!(
            await_deadline(started_at, &CancellationToken::new()).await,
            DeadlineWake::Reached
        );
        assert_eq!(started_at.elapsed(), Duration::ZERO);
    }

    #[tokio::test(start_paused = true)]
    async fn wakes_when_cancelled_before_the_deadline() {
        let started_at = Instant::now();
        let cancellation_token = CancellationToken::new();

        cancellation_token.cancel();

        assert_eq!(
            await_deadline(started_at + WAIT, &cancellation_token).await,
            DeadlineWake::Cancelled
        );
        assert_eq!(started_at.elapsed(), Duration::ZERO);
    }
}

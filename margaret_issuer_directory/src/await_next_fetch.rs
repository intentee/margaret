use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use margaret_deadline::await_deadline::await_deadline;
use margaret_deadline::deadline_wake::DeadlineWake;
use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;

pub(crate) async fn await_next_fetch(
    key_set: &IssuerKeySet,
    started_at: Instant,
    due_at: Instant,
    cancellation_token: &CancellationToken,
) -> DeadlineWake {
    tokio::select! {
        biased;
        wake = await_deadline(due_at, cancellation_token) => wake,
        () = key_set.refresh_requested() => {
            await_deadline(started_at + ISSUER_FETCH_SPACING, cancellation_token).await
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::Instant;
    use tokio::time::timeout;
    use tokio_util::sync::CancellationToken;

    use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
    use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
    use margaret_issuer_key_set::key_set_poll_interval_after_ready::KEY_SET_POLL_INTERVAL_AFTER_READY;

    use super::await_next_fetch;
    use margaret_deadline::deadline_wake::DeadlineWake;

    #[tokio::test(start_paused = true)]
    async fn waits_until_the_next_fetch_is_due() {
        let started_at = Instant::now();

        assert_eq!(
            await_next_fetch(
                &IssuerKeySet::awaiting(),
                started_at,
                started_at + KEY_SET_POLL_INTERVAL_AFTER_READY,
                &CancellationToken::new()
            )
            .await,
            DeadlineWake::Reached
        );
        assert_eq!(started_at.elapsed(), KEY_SET_POLL_INTERVAL_AFTER_READY);
    }

    #[tokio::test(start_paused = true)]
    async fn refetches_on_request_once_the_fetch_spacing_passes() {
        let key_set = IssuerKeySet::awaiting();
        let snapshot = key_set.snapshot();
        let cancellation_token = CancellationToken::new();
        let started_at = Instant::now();

        let (next_fetch, refresh) = tokio::join!(
            await_next_fetch(
                &key_set,
                started_at,
                started_at + KEY_SET_POLL_INTERVAL_AFTER_READY,
                &cancellation_token
            ),
            timeout(
                ISSUER_FETCH_SPACING / 2,
                key_set.request_refresh_after(&snapshot)
            )
        );

        assert_eq!(next_fetch, DeadlineWake::Reached);
        assert!(refresh.is_err());
        assert_eq!(started_at.elapsed(), ISSUER_FETCH_SPACING);
    }

    #[tokio::test(start_paused = true)]
    async fn stops_waiting_when_cancelled_before_the_next_fetch_is_due() {
        let cancellation_token = CancellationToken::new();
        let started_at = Instant::now();

        cancellation_token.cancel();

        assert_eq!(
            await_next_fetch(
                &IssuerKeySet::awaiting(),
                started_at,
                started_at + KEY_SET_POLL_INTERVAL_AFTER_READY,
                &cancellation_token
            )
            .await,
            DeadlineWake::Cancelled
        );
    }

    #[tokio::test(start_paused = true)]
    async fn stops_waiting_when_cancelled_during_the_fetch_spacing() {
        let key_set = IssuerKeySet::awaiting();
        let snapshot = key_set.snapshot();
        let cancellation_token = CancellationToken::new();
        let started_at = Instant::now();

        let (next_fetch, ()) = tokio::join!(
            await_next_fetch(
                &key_set,
                started_at,
                started_at + KEY_SET_POLL_INTERVAL_AFTER_READY,
                &cancellation_token
            ),
            async {
                assert!(
                    timeout(
                        ISSUER_FETCH_SPACING / 2,
                        key_set.request_refresh_after(&snapshot)
                    )
                    .await
                    .is_err()
                );
                cancellation_token.cancel();
            }
        );

        assert_eq!(next_fetch, DeadlineWake::Cancelled);
        assert_eq!(started_at.elapsed(), ISSUER_FETCH_SPACING / 2);
    }
}

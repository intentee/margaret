use tokio::time::Instant;
use tokio::time::sleep_until;
use tokio_util::sync::CancellationToken;

use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;

use crate::issuer_group::IssuerGroup;
use crate::next_fetch::NextFetch;

async fn spaced(started_at: Instant, cancellation_token: &CancellationToken) -> NextFetch {
    tokio::select! {
        biased;
        () = cancellation_token.cancelled() => NextFetch::Cancelled,
        () = sleep_until(started_at + ISSUER_FETCH_SPACING) => NextFetch::Due,
    }
}

pub(crate) async fn await_next_fetch(
    group: &IssuerGroup,
    started_at: Instant,
    due_at: Instant,
    cancellation_token: &CancellationToken,
) -> NextFetch {
    tokio::select! {
        biased;
        () = cancellation_token.cancelled() => NextFetch::Cancelled,
        () = sleep_until(due_at) => NextFetch::Due,
        () = group.refresh_requested() => spaced(started_at, cancellation_token).await,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::time::Instant;
    use tokio::time::timeout;
    use tokio_util::sync::CancellationToken;
    use url::Url;

    use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
    use margaret_issuer_key_set::key_set_poll_interval_after_ready::KEY_SET_POLL_INTERVAL_AFTER_READY;
    use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;
    use margaret_jwt_verification_tests::token_trust_declaration::TokenTrustDeclaration;
    use margaret_token_trust::token_trust::TokenTrust;
    use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

    use super::await_next_fetch;
    use crate::issuer_group::IssuerGroup;
    use crate::next_fetch::NextFetch;

    fn localhost_issuer() -> Arc<TrustedIssuer> {
        Arc::new(TrustedIssuer::for_jwks_endpoint(
            Arc::new(StaticEndpoint::new(
                Url::parse("https://localhost/jwks").expect("the fixture key set url parses"),
            )),
            Arc::new(TokenTrustDeclaration {
                trust: TokenTrust {
                    audience: "margaret".parse().expect("the audience is not empty"),
                    issuer: "https://localhost"
                        .parse()
                        .expect("the issuer is an https url"),
                },
            }),
        ))
    }

    #[tokio::test(start_paused = true)]
    async fn waits_until_the_next_fetch_is_due() {
        let started_at = Instant::now();

        assert_eq!(
            await_next_fetch(
                &IssuerGroup::founded_by(localhost_issuer()),
                started_at,
                started_at + KEY_SET_POLL_INTERVAL_AFTER_READY,
                &CancellationToken::new()
            )
            .await,
            NextFetch::Due
        );
        assert_eq!(started_at.elapsed(), KEY_SET_POLL_INTERVAL_AFTER_READY);
    }

    #[tokio::test(start_paused = true)]
    async fn refetches_on_request_once_the_fetch_spacing_passes() {
        let trusted_issuer = localhost_issuer();
        let group = IssuerGroup::founded_by(Arc::clone(&trusted_issuer));
        let snapshot = trusted_issuer.key_set.snapshot();
        let cancellation_token = CancellationToken::new();
        let started_at = Instant::now();

        let (next_fetch, refresh) = tokio::join!(
            await_next_fetch(
                &group,
                started_at,
                started_at + KEY_SET_POLL_INTERVAL_AFTER_READY,
                &cancellation_token
            ),
            timeout(
                ISSUER_FETCH_SPACING / 2,
                trusted_issuer.key_set.refreshed_since(&snapshot)
            )
        );

        assert_eq!(next_fetch, NextFetch::Due);
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
                &IssuerGroup::founded_by(localhost_issuer()),
                started_at,
                started_at + KEY_SET_POLL_INTERVAL_AFTER_READY,
                &cancellation_token
            )
            .await,
            NextFetch::Cancelled
        );
    }

    #[tokio::test(start_paused = true)]
    async fn stops_waiting_when_cancelled_during_the_fetch_spacing() {
        let trusted_issuer = localhost_issuer();
        let group = IssuerGroup::founded_by(Arc::clone(&trusted_issuer));
        let snapshot = trusted_issuer.key_set.snapshot();
        let cancellation_token = CancellationToken::new();
        let started_at = Instant::now();

        let (next_fetch, ()) = tokio::join!(
            await_next_fetch(
                &group,
                started_at,
                started_at + KEY_SET_POLL_INTERVAL_AFTER_READY,
                &cancellation_token
            ),
            async {
                assert!(
                    timeout(
                        ISSUER_FETCH_SPACING / 2,
                        trusted_issuer.key_set.refreshed_since(&snapshot)
                    )
                    .await
                    .is_err()
                );
                cancellation_token.cancel();
            }
        );

        assert_eq!(next_fetch, NextFetch::Cancelled);
        assert_eq!(started_at.elapsed(), ISSUER_FETCH_SPACING / 2);
    }
}

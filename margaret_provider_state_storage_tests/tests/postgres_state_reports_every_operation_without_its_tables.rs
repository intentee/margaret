use uuid::Uuid;

use margaret_provider_state_storage::pending_decision::PendingDecision;
use margaret_provider_state_storage::pending_verdict::PendingVerdict;
use margaret_provider_state_storage::provider_state_error::ProviderStateError;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_provider_state_storage_tests::assertion_clock::AssertionClock;
use margaret_provider_state_storage_tests::fixture_grant::fixture_grant;
use margaret_provider_state_storage_tests::fresh_digest::fresh_digest;
use margaret_provider_state_storage_tests::pending_of::pending_of;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;

#[tokio::test]
async fn postgres_state_reports_every_operation_without_its_tables() {
    let postgres = PostgresState::with_tables().await;
    let state = &postgres.state;
    let grant = fixture_grant();
    let code = fresh_digest();
    let refresh_token = fresh_digest();
    let clock = AssertionClock::start();

    postgres.dropped().await;

    assert!(matches!(
        state.issue_code(code, grant.clone()).await,
        Err(ProviderStateError::IssueCode { .. })
    ));
    assert!(matches!(
        state.present_code(code).await,
        Err(ProviderStateError::PresentCode { .. })
    ));
    assert!(matches!(
        state.spend_code(code, RefreshIssuance::Withheld).await,
        Err(ProviderStateError::SpendCode { .. })
    ));
    assert!(matches!(
        state
            .hold_pending_authorization(Uuid::new_v4(), pending_of())
            .await,
        Err(ProviderStateError::HoldPendingAuthorization { .. })
    ));
    assert!(matches!(
        state
            .decide_pending_authorization(
                Uuid::new_v4(),
                PendingDecision {
                    subject: grant.subject,
                    verdict: PendingVerdict::Denied,
                },
            )
            .await,
        Err(ProviderStateError::DecidePendingAuthorization { .. })
    ));
    assert!(matches!(
        state.present_refresh_token(refresh_token).await,
        Err(ProviderStateError::PresentRefreshToken { .. })
    ));
    assert!(matches!(
        state
            .rotate_refresh_token(refresh_token, fresh_digest())
            .await,
        Err(ProviderStateError::RotateRefreshToken { .. })
    ));
    assert!(matches!(
        state
            .revoke_refresh_token(refresh_token, &grant.client_id)
            .await,
        Err(ProviderStateError::RevokeRefreshToken { .. })
    ));
    assert!(matches!(
        state
            .spend_client_assertion("portal", code, clock.in_seconds(60), clock.now)
            .await,
        Err(ProviderStateError::SpendClientAssertion { .. })
    ));
}

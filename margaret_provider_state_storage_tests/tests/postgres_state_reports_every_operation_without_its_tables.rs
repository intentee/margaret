use uuid::Uuid;

use margaret_provider_state_storage::code_redemption_request::CodeRedemptionRequest;
use margaret_provider_state_storage::pending_decision::PendingDecision;
use margaret_provider_state_storage::pending_verdict::PendingVerdict;
use margaret_provider_state_storage::provider_state_error::ProviderStateError;
use margaret_provider_state_storage::refresh_admission::RefreshAdmission;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::refresh_scope::RefreshScope;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_provider_state_storage_tests::admission_of::admission_of;
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
    let redemption = || CodeRedemptionRequest {
        admission: admission_of(&grant),
        family: Uuid::new_v4(),
        refresh: RefreshIssuance::Withheld,
    };
    let refresh = || RefreshAdmission {
        client_id: &grant.client_id,
        scope: &RefreshScope::Granted,
    };
    let refresh_token = fresh_digest();

    postgres.dropped().await;

    assert!(matches!(
        state.issue_code(code, grant.clone()).await,
        Err(ProviderStateError::IssueCode { .. })
    ));
    assert!(matches!(
        state.redeem_code(code, redemption()).await,
        Err(ProviderStateError::RedeemCode { .. })
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
        state
            .rotate_refresh_token(refresh_token, fresh_digest(), refresh())
            .await,
        Err(ProviderStateError::RotateRefreshToken { .. })
    ));
    assert!(matches!(
        state
            .revoke_refresh_token(refresh_token, &grant.client_id)
            .await,
        Err(ProviderStateError::RevokeRefreshToken { .. })
    ));
}

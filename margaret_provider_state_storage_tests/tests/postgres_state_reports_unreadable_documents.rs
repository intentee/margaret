use serde_json::json;
use sqlx::query;
use uuid::Uuid;

use margaret_provider_state_storage::code_redemption_request::CodeRedemptionRequest;
use margaret_provider_state_storage::pending_decision::PendingDecision;
use margaret_provider_state_storage::pending_verdict::PendingVerdict;
use margaret_provider_state_storage::provider_state_error::ProviderStateError;
use margaret_provider_state_storage::refresh_admission::RefreshAdmission;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::refresh_scope::RefreshScope;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_provider_state_storage::token_digest::TokenDigest;
use margaret_provider_state_storage_tests::admission_of::admission_of;
use margaret_provider_state_storage_tests::fixture_client_id::fixture_client_id;
use margaret_provider_state_storage_tests::fixture_grant::fixture_grant;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;

async fn stored(postgres: &PostgresState, statement: &'static str, key: Vec<u8>, document: String) {
    query(statement)
        .bind(key)
        .bind(document)
        .execute(postgres.database.pool())
        .await
        .expect("the incomplete document is stored");
}

#[tokio::test]
async fn postgres_state_reports_unreadable_documents() {
    let postgres = PostgresState::with_tables().await;
    let state = &postgres.state;
    let grant = fixture_grant();
    let code = TokenDigest::of("code");
    let redemption = || CodeRedemptionRequest {
        admission: admission_of(&grant),
        family: Uuid::new_v4(),
        refresh: RefreshIssuance::Withheld,
    };
    let refresh = || RefreshAdmission {
        client_id: &grant.client_id,
        scope: &RefreshScope::Granted,
    };
    let pending_id = Uuid::new_v4();
    let family = Uuid::new_v4();
    let refresh_token = TokenDigest::of("refresh");

    stored(
        &postgres,
        r#"INSERT INTO "margaret-oidc-authorization-codes" ("digest", "grant_document", "redeemed_family", "expires_at") VALUES ($1, $2, NULL, now() + interval '1 minute')"#,
        code.as_bytes().to_vec(),
        json!({
            "client_id": grant.client_id.as_str(),
            "code_challenge": grant.code_challenge,
            "redirect_uri": grant.redirect_uri.as_str(),
        })
        .to_string(),
    )
    .await;
    query(
        r#"INSERT INTO "margaret-oidc-pending-authorizations" ("id", "pending_document", "expires_at") VALUES ($1, $2, now() + interval '1 minute')"#,
    )
    .bind(pending_id)
    .bind(json!({"grant": {"subject": grant.subject.to_string()}}).to_string())
    .execute(postgres.database.pool())
    .await
    .expect("the incomplete pending authorization is stored");
    query(
        r#"INSERT INTO "margaret-oidc-refresh-families" ("id", "grant_document", "expires_at") VALUES ($1, $2, now() + interval '1 minute')"#,
    )
    .bind(family)
    .bind(json!({"client_id": fixture_client_id().as_str(), "scopes": []}).to_string())
    .execute(postgres.database.pool())
    .await
    .expect("the incomplete family is stored");
    stored(
        &postgres,
        r#"INSERT INTO "margaret-oidc-refresh-tokens" ("digest", "family", "superseded", "expires_at") SELECT $1, "id", FALSE, now() + interval '1 minute' FROM "margaret-oidc-refresh-families" WHERE "grant_document" = $2"#,
        refresh_token.as_bytes().to_vec(),
        json!({"client_id": fixture_client_id().as_str(), "scopes": []}).to_string(),
    )
    .await;

    assert!(matches!(
        state.redeem_code(code, redemption()).await,
        Err(ProviderStateError::RedeemCode { .. })
    ));
    assert!(matches!(
        state
            .decide_pending_authorization(
                pending_id,
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
            .rotate_refresh_token(refresh_token, TokenDigest::of("next"), refresh())
            .await,
        Err(ProviderStateError::RotateRefreshToken { .. })
    ));
}

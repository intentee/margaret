use std::collections::BTreeSet;

use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oidc_provider_tests::end_user_subject::END_USER_SUBJECT;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

const EXPIRED_REFRESH_TOKEN: &str = "a-refresh-token-of-an-expired-family";

#[tokio::test]
async fn refuses_an_expired_refresh_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let now = Utc::now();

    RefreshFamilyRecord::open(
        &fixture.storage.database,
        Uuid::new_v4(),
        RefreshFamily {
            auth_time: now,
            client_id: "portal".to_string(),
            expires_at: NumericDate::from(now),
            scopes: BTreeSet::from([Scope::openid()]),
            subject: END_USER_SUBJECT,
        },
        TokenDigest::of(EXPIRED_REFRESH_TOKEN),
        NumericDate::from(now),
    )
    .await
    .expect("the fixture database opens the family");

    let answer = refreshed_tokens(&fixture, &json!(EXPIRED_REFRESH_TOKEN), None).await;

    assert_eq!(answer.status, 400);
    assert_eq!(
        answer.body["error_description"],
        "the refresh token is not known"
    );

    fixture.stop().await;
}

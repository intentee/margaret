use chrono::Utc;

use margaret_authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::fixture_authorization_grant::fixture_authorization_grant;
use margaret_oidc_provider_tests::portal_callback::PORTAL_CALLBACK;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

const EXPIRED_CODE: &str = "an-authorization-code-that-expired";

#[tokio::test]
async fn refuses_an_expired_authorization_code() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let now = NumericDate::from(Utc::now());

    AuthorizationCodeRecord::issue(
        &fixture.storage.database,
        TokenDigest::of(EXPIRED_CODE),
        IssuedCode {
            expires_at: now,
            grant: fixture_authorization_grant("portal", PORTAL_CALLBACK),
        },
        now,
    )
    .await
    .expect("the fixture database issues the code");

    let answer = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(EXPIRED_CODE))
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(
        answer.body["error_description"],
        "the authorization code is not known"
    );

    fixture.stop().await;
}

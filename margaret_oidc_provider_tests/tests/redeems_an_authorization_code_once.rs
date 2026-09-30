use serde_json::json;

use crate::code_exchange::code_exchange;
use crate::issued_code::issued_code;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn redeems_an_authorization_code_once() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let code = issued_code(&fixture, &portal_parameters()).await;
    let first = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
        .await;
    let replayed = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
        .await;
    let refreshed = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &json!({
                "grant_type": "refresh_token",
                "refresh_token": first.body["refresh_token"],
            }),
        )
        .await;

    assert_eq!(first.status, 200);
    assert_eq!(replayed.status, 400);
    assert_eq!(replayed.body["error"], "invalid_grant");
    assert_eq!(refreshed.body["error"], "invalid_grant");

    fixture.stop().await;
}

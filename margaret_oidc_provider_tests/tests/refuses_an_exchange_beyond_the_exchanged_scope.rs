use serde_json::Value;

use crate::ci_issuer::CiIssuer;
use crate::exchange_request::exchange_request;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_an_exchange_beyond_the_exchanged_scope() {
    let ci = CiIssuer::publishing_keys();
    let fixture = ProviderFixture::start(vec![ci.exchanger.clone()]).await;
    let mut request = exchange_request(&ci.token("intentee/margaret"));

    request["scope"] = Value::String("openid".to_string());

    let answer = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &request)
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_scope");

    fixture.stop().await;
}

use serde_json::json;

use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_an_unsupported_subject_token_type() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture.post_form("/token", &PORTAL_CREDENTIALS, &json!({"grant_type": "urn:ietf:params:oauth:grant-type:token-exchange", "subject_token": "a.b.c", "subject_token_type": "urn:ietf:params:oauth:token-type:saml2"})).await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_request");

    fixture.stop().await;
}

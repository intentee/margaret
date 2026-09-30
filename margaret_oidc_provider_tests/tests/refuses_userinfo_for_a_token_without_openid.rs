use crate::answer::Answer;
use crate::client_credentials::ClientCredentials;
use crate::code_exchange::code_exchange;
use crate::issued_code::issued_code;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::provider_url::provider_url;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn refuses_userinfo_for_a_token_without_openid() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let code = issued_code(
        &fixture,
        &with_parameter(portal_parameters(), "scope", "profile"),
    )
    .await;
    let access_token = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
        .await
        .body["access_token"]
        .as_str()
        .expect("the access token is a string")
        .to_string();
    let answer = Answer::of(
        ClientCredentials::Bearer(access_token)
            .presented_on(fixture.client.get(provider_url("/userinfo")))
            .send()
            .await
            .expect("the provider answers"),
    )
    .await;

    assert_eq!(answer.status, 401);

    fixture.stop().await;
}

use crate::code_exchange::code_exchange;
use crate::issued_code::issued_code;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::service_credentials::SERVICE_CREDENTIALS;

#[tokio::test]
async fn refuses_a_code_exchanged_by_a_client_without_the_grant() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let code = issued_code(&fixture, &portal_parameters()).await;
    let answer = fixture
        .post_form("/token", &SERVICE_CREDENTIALS, &code_exchange(&code))
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "unauthorized_client");

    fixture.stop().await;
}

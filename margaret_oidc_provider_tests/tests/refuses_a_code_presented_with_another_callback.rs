use crate::code_exchange::code_exchange;
use crate::issued_code::issued_code;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn refuses_a_code_presented_with_another_callback() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let code = issued_code(&fixture, &portal_parameters()).await;
    let answer = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &with_parameter(
                code_exchange(&code),
                "redirect_uri",
                "https://portal.localhost/other",
            ),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_grant");

    fixture.stop().await;
}

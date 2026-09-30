use crate::code_exchange::code_exchange;
use crate::issued_code::issued_code;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn refuses_a_code_presented_with_a_malformed_callback() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let code = issued_code(&fixture, &portal_parameters()).await;
    let malformed = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &with_parameter(code_exchange(&code), "redirect_uri", "not a url"),
        )
        .await;
    let exchanged = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
        .await;

    assert_eq!(malformed.status, 400);
    assert_eq!(malformed.body["error"], "invalid_grant");
    assert_eq!(exchanged.status, 200);

    fixture.stop().await;
}

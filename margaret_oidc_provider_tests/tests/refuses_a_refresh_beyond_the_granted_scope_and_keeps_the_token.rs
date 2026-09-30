use crate::code_exchange::code_exchange;
use crate::issued_code::issued_code;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::refreshed_tokens::refreshed_tokens;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn refuses_a_refresh_beyond_the_granted_scope_and_keeps_the_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let code = issued_code(
        &fixture,
        &with_parameter(portal_parameters(), "scope", "openid"),
    )
    .await;
    let refresh_token = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
        .await
        .body["refresh_token"]
        .clone();
    let exceeding = refreshed_tokens(&fixture, &refresh_token, Some("openid profile")).await;

    assert_eq!(exceeding.status, 400);
    assert_eq!(exceeding.body["error"], "invalid_scope");
    assert_eq!(
        refreshed_tokens(&fixture, &refresh_token, None)
            .await
            .status,
        200
    );

    fixture.stop().await;
}

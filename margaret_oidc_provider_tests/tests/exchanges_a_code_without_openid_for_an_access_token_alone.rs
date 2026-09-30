use crate::code_exchange::code_exchange;
use crate::issued_code::issued_code;
use crate::jwt_parts::JwtParts;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn exchanges_a_code_without_openid_for_an_access_token_alone() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let code = issued_code(
        &fixture,
        &with_parameter(portal_parameters(), "scope", "profile"),
    )
    .await;
    let answer = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
        .await;

    assert_eq!(answer.status, 200);
    assert!(answer.body.get("id_token").is_none());
    assert_eq!(
        JwtParts::of(&answer.body["access_token"]).payload["aud"],
        "artifacts"
    );

    fixture.stop().await;
}

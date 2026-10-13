use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::constants::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_an_unknown_code_as_invalid_grant_so_only_issued_codes_yield_tokens() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange("unknown"))
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_grant");

    fixture.stop().await;
}

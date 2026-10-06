use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::rfc_example_client::rfc_example_client;

const AUTHORIZATION: &str = include_str!(
    "../../margaret_oauth_vocabulary/fixtures/rfc6749/section_2_3_1_authorization.txt"
);
const REQUEST_BODY: &str = include_str!("../fixtures/rfc7009/section_2_1_request_body.txt");

#[tokio::test]
async fn accepts_the_rfc_7009_example_revocation_of_an_invalid_token() {
    let fixture = ProviderFixture::serving(vec![rfc_example_client()], Vec::new()).await;
    let answer = fixture
        .post_encoded_form("/revoke", AUTHORIZATION, REQUEST_BODY)
        .await;

    assert_eq!(answer.status, 200);

    fixture.stop().await;
}

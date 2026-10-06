use serde_json::Value;

use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::rfc_example_client::rfc_example_client;

const AUTHORIZATION: &str = include_str!(
    "../../margaret_oauth_vocabulary/fixtures/rfc6749/section_2_3_1_authorization.txt"
);
const INACTIVE_RESPONSE: &str =
    include_str!("../fixtures/rfc7662/section_2_2_inactive_response.json");
const REQUEST_BODY: &str = include_str!("../fixtures/rfc7662/section_2_1_request_body.txt");

#[tokio::test]
async fn answers_the_rfc_7662_example_introspection_of_an_inactive_token() {
    let fixture = ProviderFixture::serving(vec![rfc_example_client()], Vec::new()).await;
    let answer = fixture
        .post_encoded_form("/introspect", AUTHORIZATION, REQUEST_BODY)
        .await;

    assert_eq!(answer.status, 200);
    assert_eq!(
        answer.body,
        serde_json::from_str::<Value>(INACTIVE_RESPONSE).expect("the example response is json")
    );

    fixture.stop().await;
}

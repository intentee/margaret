use crate::code_exchange::code_exchange;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;
use crate::spa_code::spa_code;

#[tokio::test]
async fn refuses_a_code_granted_to_another_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let code = spa_code(&fixture).await;
    let answer = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_grant");

    fixture.stop().await;
}

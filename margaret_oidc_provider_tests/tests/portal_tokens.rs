use crate::answer::Answer;
use crate::code_exchange::code_exchange;
use crate::issued_code::issued_code;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;

pub async fn portal_tokens(fixture: &ProviderFixture) -> Answer {
    let code = issued_code(fixture, &portal_parameters()).await;
    let answer = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
        .await;

    assert_eq!(answer.status, 200);

    answer
}

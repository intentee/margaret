use crate::answer::Answer;
use crate::code_exchange::code_exchange;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;

/// # Panics
///
/// Panics when the provider does not exchange the code for tokens.
pub async fn portal_tokens(fixture: &ProviderFixture, code: &str) -> Answer {
    let answer = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(code))
        .await;

    assert_eq!(answer.status, 200);

    answer
}

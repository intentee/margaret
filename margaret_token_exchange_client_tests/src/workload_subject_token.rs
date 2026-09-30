use zeroize::Zeroizing;

use margaret_token_exchange_client::subject_token::SubjectToken;
use margaret_token_exchange_client::subject_token_type::SubjectTokenType;

#[must_use]
pub fn workload_subject_token() -> SubjectToken {
    SubjectToken {
        token: Zeroizing::new("workload.identity.token".to_string()),
        token_type: SubjectTokenType::IdToken,
    }
}

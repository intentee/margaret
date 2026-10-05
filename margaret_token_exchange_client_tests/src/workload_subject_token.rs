use zeroize::Zeroizing;

use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_token_exchange_client::subject_token::SubjectToken;

#[must_use]
pub fn workload_subject_token() -> SubjectToken {
    SubjectToken {
        token: Zeroizing::new("workload.identity.token".to_string()),
        token_type: SubjectTokenType::IdToken,
    }
}

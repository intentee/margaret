use zeroize::Zeroizing;

use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;

pub struct SubjectToken {
    pub token: Zeroizing<String>,
    pub token_type: SubjectTokenType,
}

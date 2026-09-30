use zeroize::Zeroizing;

use crate::subject_token_type::SubjectTokenType;

pub struct SubjectToken {
    pub token: Zeroizing<String>,
    pub token_type: SubjectTokenType,
}

use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_jwt_verification::jwt_profile::JwtProfile;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;

pub trait SubjectTokenProfile: JwtProfile {
    fn admits(token_type: SubjectTokenType) -> bool;
}

impl SubjectTokenProfile for AccessTokenProfile {
    fn admits(token_type: SubjectTokenType) -> bool {
        token_type == SubjectTokenType::AccessToken
    }
}

impl SubjectTokenProfile for IdTokenProfile {
    fn admits(token_type: SubjectTokenType) -> bool {
        matches!(
            token_type,
            SubjectTokenType::IdToken | SubjectTokenType::Jwt
        )
    }
}

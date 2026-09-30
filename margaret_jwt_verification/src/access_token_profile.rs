use margaret_jose_parameters::jwt_type::JwtType;

use crate::bearer_token_profile::BearerTokenProfile;
use crate::jwt_profile::JwtProfile;
use crate::jwt_profile_seal::JwtProfileSeal;
use crate::type_header_expectation::TypeHeaderExpectation;

pub struct AccessTokenProfile;

impl BearerTokenProfile for AccessTokenProfile {}

impl JwtProfile for AccessTokenProfile {
    const TOKEN_TYPE: TypeHeaderExpectation = TypeHeaderExpectation::Required(JwtType::AccessToken);
}

impl JwtProfileSeal for AccessTokenProfile {}

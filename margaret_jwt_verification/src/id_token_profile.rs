use margaret_jose_parameters::jwt_type::JwtType;

use crate::bearer_token_profile::BearerTokenProfile;
use crate::jwt_profile::JwtProfile;
use crate::jwt_profile_seal::JwtProfileSeal;
use crate::type_header_expectation::TypeHeaderExpectation;

pub struct IdTokenProfile;

impl BearerTokenProfile for IdTokenProfile {}

impl JwtProfile for IdTokenProfile {
    const TOKEN_TYPE: TypeHeaderExpectation = TypeHeaderExpectation::Optional(JwtType::Jwt);
}

impl JwtProfileSeal for IdTokenProfile {}

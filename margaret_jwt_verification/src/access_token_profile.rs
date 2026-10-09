use margaret_jose_parameters::jwt_type::JwtType;

use crate::bearer_token_profile::BearerTokenProfile;
use crate::issued_at_requirement::IssuedAtRequirement;
use crate::jwt_profile::JwtProfile;
use crate::jwt_profile_seal::JwtProfileSeal;
use crate::type_header_expectation::TypeHeaderExpectation;

pub struct AccessTokenProfile;

impl BearerTokenProfile for AccessTokenProfile {}

impl JwtProfile for AccessTokenProfile {
    const ISSUED_AT: IssuedAtRequirement = IssuedAtRequirement::Required;
    const TOKEN_TYPE: TypeHeaderExpectation = TypeHeaderExpectation::Required(JwtType::AccessToken);
}

impl JwtProfileSeal for AccessTokenProfile {}

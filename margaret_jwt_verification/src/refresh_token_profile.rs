use margaret_jose_parameters::jwt_type::JwtType;

use crate::issued_at_requirement::IssuedAtRequirement;
use crate::jwt_profile::JwtProfile;
use crate::jwt_profile_seal::JwtProfileSeal;
use crate::type_header_expectation::TypeHeaderExpectation;

pub struct RefreshTokenProfile;

impl JwtProfile for RefreshTokenProfile {
    const ISSUED_AT: IssuedAtRequirement = IssuedAtRequirement::Required;
    const TOKEN_TYPE: TypeHeaderExpectation = TypeHeaderExpectation::Required(JwtType::Refresh);
}

impl JwtProfileSeal for RefreshTokenProfile {}

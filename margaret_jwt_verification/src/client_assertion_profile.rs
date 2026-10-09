use margaret_jose_parameters::jwt_type::JwtType;

use crate::issued_at_requirement::IssuedAtRequirement;
use crate::jwt_profile::JwtProfile;
use crate::jwt_profile_seal::JwtProfileSeal;
use crate::type_header_expectation::TypeHeaderExpectation;

pub struct ClientAssertionProfile;

impl JwtProfile for ClientAssertionProfile {
    const ISSUED_AT: IssuedAtRequirement = IssuedAtRequirement::Optional;
    const TOKEN_TYPE: TypeHeaderExpectation =
        TypeHeaderExpectation::UntypedOr(&[JwtType::ClientAuthentication, JwtType::Jwt]);
}

impl JwtProfileSeal for ClientAssertionProfile {}

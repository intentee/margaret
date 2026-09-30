use margaret_jose_parameters::jwt_type::JwtType;

use crate::jwt_profile::JwtProfile;
use crate::jwt_profile_seal::JwtProfileSeal;
use crate::type_header_expectation::TypeHeaderExpectation;

pub struct SignInTransactionProfile;

impl JwtProfile for SignInTransactionProfile {
    const TOKEN_TYPE: TypeHeaderExpectation =
        TypeHeaderExpectation::Required(JwtType::SignInTransaction);
}

impl JwtProfileSeal for SignInTransactionProfile {}

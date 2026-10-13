use crate::issued_at_requirement::IssuedAtRequirement;
use crate::jwt_profile_seal::JwtProfileSeal;
use crate::type_header_expectation::TypeHeaderExpectation;

pub trait JwtProfile: JwtProfileSeal + Send + Sync + 'static {
    const ISSUED_AT: IssuedAtRequirement;
    const TOKEN_TYPE: TypeHeaderExpectation;
}

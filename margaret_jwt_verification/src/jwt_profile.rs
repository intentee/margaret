use crate::jwt_profile_seal::JwtProfileSeal;
use crate::type_header_expectation::TypeHeaderExpectation;

pub trait JwtProfile: JwtProfileSeal + Send + Sync + 'static {
    const TOKEN_TYPE: TypeHeaderExpectation;
}

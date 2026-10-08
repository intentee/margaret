use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_registered_claims::numeric_date::NumericDate;

#[derive(Debug)]
pub enum ClientRefusal {
    AssertionIdentifierMissing,
    AssertionMissing,
    AssertionOutlivesLimit {
        exp: NumericDate,
        limit: NumericDate,
    },
    AssertionReplayed,
    AssertionRejected(JwtRejection),
    AssertionRequired,
    AssertionTypeMissing,
    ConflictingClientIds,
    HeaderAuthentication,
    MissingCredentials,
    PublicClientAssertion,
    SubjectMismatch {
        found: String,
    },
    UnknownClient,
    UnsupportedAssertionType,
}

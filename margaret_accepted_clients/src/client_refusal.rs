use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_provider_state_storage::assertion_refusal::AssertionRefusal;
use margaret_registered_claims::numeric_date::NumericDate;

#[derive(Debug)]
pub enum ClientRefusal {
    AssertionIdentifierMissing,
    AssertionMissing,
    AssertionOutlivesLimit {
        exp: NumericDate,
        limit: NumericDate,
    },
    AssertionRefused(AssertionRefusal),
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

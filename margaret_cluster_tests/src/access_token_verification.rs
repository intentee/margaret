use chrono::Utc;
use serde_json::Value;

use margaret::framework::jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile;
use margaret::framework::jwt_verification::expected_audience::ExpectedAudience;
use margaret::framework::jwt_verification::jwt_expectation::JwtExpectation;
use margaret::framework::jwt_verification::jwt_verification::JwtVerification;
use margaret::framework::jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret::framework::registered_claims::numeric_date::NumericDate;
use margaret_cluster_fixture::margaret::token_issuance::TOKEN_ISSUANCE;

#[must_use]
pub fn access_token_verification(
    document: &AcceptedKeySetDocument,
    access_token: &str,
    audience: &str,
) -> JwtVerification<Value, AccessTokenProfile> {
    verify_serialized_jwt::<Value, AccessTokenProfile>(
        &document.key_set,
        access_token,
        &JwtExpectation {
            audience: ExpectedAudience::One(audience),
            issuer: TOKEN_ISSUANCE.issuer,
        },
        NumericDate::from(Utc::now()),
    )
}

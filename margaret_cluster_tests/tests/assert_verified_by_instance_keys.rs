use chrono::Utc;
use serde_json::Value;

use margaret::framework::jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret::framework::jws_verification::verification_key_set::VerificationKeySet;
use margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile;
use margaret::framework::jwt_verification::expected_audience::ExpectedAudience;
use margaret::framework::jwt_verification::jwt_expectation::JwtExpectation;
use margaret::framework::jwt_verification::jwt_verification::JwtVerification;
use margaret::framework::jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret::framework::registered_claims::numeric_date::NumericDate;
use margaret_cluster_fixture::margaret::token_issuance::TOKEN_ISSUANCE;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::instance_key_set::instance_key_set;

pub async fn assert_verified_by_instance_keys(
    cluster: &Cluster,
    index: usize,
    access_token: &str,
    audience: &str,
) {
    let KeySetDocumentParsing::Accepted(document) =
        VerificationKeySet::parse(&instance_key_set(cluster, index).await)
    else {
        panic!("the instance publishes a key set");
    };

    if let JwtVerification::Rejected(rejection) = verify_serialized_jwt::<Value, AccessTokenProfile>(
        &document.key_set,
        access_token,
        &JwtExpectation {
            audience: ExpectedAudience::One(audience),
            issuer: TOKEN_ISSUANCE.issuer,
        },
        NumericDate::from(Utc::now()),
    ) {
        panic!("the instance keys reject the access token: {rejection:?}");
    }
}

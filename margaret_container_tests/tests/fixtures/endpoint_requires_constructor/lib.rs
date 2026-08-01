use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret::framework::jwt_claims::provides_expected_claims::ProvidesExpectedClaims;

#[provides_jwks_endpoint(jwks)]
#[singleton]
struct JwksEndpoint {
    url: String,
}

impl ProvidesEndpoint for JwksEndpoint {}

impl ProvidesExpectedClaims for JwksEndpoint {}

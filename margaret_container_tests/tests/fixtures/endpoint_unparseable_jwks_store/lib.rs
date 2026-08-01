use std::sync::Arc;

use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret::framework::jwt_claims::provides_expected_claims::ProvidesExpectedClaims;

#[provides_jwks_endpoint(jwks)]
#[singleton]
struct JwksEndpoint;

impl JwksEndpoint {
    #[constructor]
    fn new(#[jwks_secret_store(= 5)] store: Arc<ServerCapability>) -> anyhow::Result<Self> {}
}

impl ProvidesEndpoint for JwksEndpoint {}

impl ProvidesExpectedClaims for JwksEndpoint {}

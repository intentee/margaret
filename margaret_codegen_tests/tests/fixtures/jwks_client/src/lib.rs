use std::sync::Arc;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_http::response::Response;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;

#[singleton]
#[provides_endpoint(issuer)]
struct IssuerEndpoint;

impl IssuerEndpoint {
    #[constructor]
    fn create() -> Self {}
}

impl ProvidesEndpoint for IssuerEndpoint {}

#[singleton]
#[responds_to_http(method = "get", path = "/verify", server = "public")]
struct Verify {
    verifier: Arc<PublicJwksVerifier>,
}

impl Verify {
    #[constructor]
    fn create(verifier: Arc<PublicJwksVerifier>) -> Self {}

    #[process]
    fn respond(&self) -> Response {}
}

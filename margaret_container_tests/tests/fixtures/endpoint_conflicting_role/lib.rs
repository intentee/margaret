use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

#[provides_jwks_endpoint(jwks)]
#[singleton]
#[service]
struct JwksEndpoint;

impl JwksEndpoint {
    #[constructor]
    fn new() -> Self {}
}

impl ProvidesEndpoint for JwksEndpoint {}

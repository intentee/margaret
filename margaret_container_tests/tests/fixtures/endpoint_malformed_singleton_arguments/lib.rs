use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

#[provides_endpoint(jwks)]
#[singleton(= 5)]
struct JwksEndpoint;

impl JwksEndpoint {
    #[constructor]
    fn new() -> Self {}
}

impl ProvidesEndpoint for JwksEndpoint {}

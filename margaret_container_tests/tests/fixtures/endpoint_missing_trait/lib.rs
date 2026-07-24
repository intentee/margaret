#[provides_endpoint(jwks)]
#[singleton]
struct JwksEndpoint;

impl JwksEndpoint {
    #[constructor]
    fn new() -> Self {}
}

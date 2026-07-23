#[provides_endpoint(jwks)]
struct JwksEndpoint;

impl JwksEndpoint {
    #[constructor]
    fn new() -> Self {}
}

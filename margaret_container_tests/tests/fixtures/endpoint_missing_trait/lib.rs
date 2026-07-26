#[provides_jwks_endpoint(jwks)]
#[singleton]
struct JwksEndpoint;

impl JwksEndpoint {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

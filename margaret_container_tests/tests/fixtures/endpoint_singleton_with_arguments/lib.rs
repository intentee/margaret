use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;

#[provides_jwks_endpoint(jwks)]
#[singleton(unexpected = Thing)]
struct JwksEndpoint;

impl JwksEndpoint {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

impl ProvidesEndpoint for JwksEndpoint {}

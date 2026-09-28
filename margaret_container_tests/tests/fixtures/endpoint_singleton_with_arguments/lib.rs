use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;

#[provides_jwks_endpoint(jwks)]
#[singleton(unexpected = Thing)]
struct JwksEndpoint;

impl JwksEndpoint {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

impl ProvidesEndpoint for JwksEndpoint {}

impl DeclaresTokenTrust for JwksEndpoint {}

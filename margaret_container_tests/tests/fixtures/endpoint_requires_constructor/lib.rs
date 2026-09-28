use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;

#[provides_jwks_endpoint(jwks)]
#[singleton]
struct JwksEndpoint {
    url: String,
}

impl ProvidesEndpoint for JwksEndpoint {}

impl DeclaresTokenTrust for JwksEndpoint {}

use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;

#[provides_jwks_endpoint(jwks)]
#[singleton]
struct JwksEndpoint {
    url: String,
}

impl ProvidesEndpoint for JwksEndpoint {}

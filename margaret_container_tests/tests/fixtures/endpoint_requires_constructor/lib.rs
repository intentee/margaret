use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

#[provides_endpoint(jwks)]
#[singleton]
struct JwksEndpoint {
    url: String,
}

impl ProvidesEndpoint for JwksEndpoint {}

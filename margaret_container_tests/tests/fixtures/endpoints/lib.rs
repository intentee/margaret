use std::sync::Arc;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

trait Resolver {}

#[singleton(provides = Resolver)]
struct DnsResolver;

impl DnsResolver {
    #[constructor]
    fn new() -> Self {}
}

impl Resolver for DnsResolver {}

#[provides_endpoint(jwks)]
struct JwksEndpoint;

impl JwksEndpoint {
    #[constructor]
    fn new(
        resolver: Arc<dyn Resolver>,
        #[console_argument(from = "issuer")] issuer: String,
    ) -> Self {}
}

impl ProvidesEndpoint for JwksEndpoint {}

#[singleton]
struct JwksClient;

impl JwksClient {
    #[constructor]
    fn new(#[endpoint_provider(jwks)] issuer: Arc<dyn ProvidesEndpoint>) -> Self {}
}

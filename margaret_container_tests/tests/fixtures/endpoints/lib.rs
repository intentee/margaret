use std::sync::Arc;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

#[singleton]
struct DnsResolver;

impl DnsResolver {
    #[constructor]
    fn new() -> Self {}
}

#[provides_endpoint(jwks)]
#[singleton]
struct JwksEndpoint;

impl JwksEndpoint {
    #[constructor]
    fn new(
        resolver: Arc<DnsResolver>,
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

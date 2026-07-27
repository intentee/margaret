use std::sync::Arc;

use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;

#[singleton]
struct DnsResolver;

impl DnsResolver {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

#[provides_jwks_endpoint(jwks)]
#[singleton]
struct JwksEndpoint;

impl JwksEndpoint {
    #[constructor]
    fn new(
        resolver: Arc<DnsResolver>,
        #[console_argument(from = "issuer")] issuer: String,
    ) -> anyhow::Result<Self> {}
}

impl ProvidesEndpoint for JwksEndpoint {}

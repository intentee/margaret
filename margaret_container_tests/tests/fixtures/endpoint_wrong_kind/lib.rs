use std::sync::Arc;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

#[singleton]
#[handles_middleware_attribute(attribute = logged)]
struct RequestLog;

impl RequestLog {
    #[constructor]
    fn new() -> Self {}
}

#[singleton]
struct Client;

impl Client {
    #[constructor]
    fn new(#[endpoint_provider(logged)] issuer: Arc<dyn ProvidesEndpoint>) -> Self {}
}

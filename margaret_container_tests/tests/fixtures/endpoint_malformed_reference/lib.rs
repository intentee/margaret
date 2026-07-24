use std::sync::Arc;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

#[singleton]
struct Client;

impl Client {
    #[constructor]
    fn new(#[endpoint_provider(endpoints::jwks)] issuer: Arc<dyn ProvidesEndpoint>) -> Self {}
}

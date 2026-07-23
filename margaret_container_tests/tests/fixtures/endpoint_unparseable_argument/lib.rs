use std::sync::Arc;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

#[singleton]
struct Client;

impl Client {
    #[constructor]
    fn new(#[endpoint_provider(= 5)] issuer: Arc<dyn ProvidesEndpoint>) -> Self {}
}

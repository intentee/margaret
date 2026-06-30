use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::config::Config;

#[singleton]
#[responds_to_http(method = Get, path = "/health", server = crate::servers::public::Public)]
pub struct GetHealth {
    config: Arc<Config>,
}

impl GetHealth {
    #[constructor]
    pub fn create(config: Arc<Config>) -> Self {
        Self { config }
    }

    #[responder]
    pub async fn respond(&self) -> Response {
        Response::text(200, self.config.app_name())
    }
}

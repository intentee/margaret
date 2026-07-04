use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::config::Config;

#[singleton]
#[responds_to_http(method = "get", path = "/health", server = "public")]
pub struct GetHealth {
    config: Arc<Config>,
}

impl GetHealth {
    #[constructor]
    pub fn create(config: Arc<Config>) -> Self {
        Self { config }
    }

    #[process]
    pub async fn respond(&self) -> Response {
        Response::text(200, self.config.app_name())
    }
}

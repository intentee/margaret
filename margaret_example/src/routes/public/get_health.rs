use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::app_name::AppName;

#[singleton]
#[responds_to_http(method = "get", path = "/health", server = "public")]
pub struct GetHealth {
    app_name: Arc<AppName>,
}

impl GetHealth {
    #[constructor]
    #[must_use]
    pub fn create(app_name: Arc<AppName>) -> Self {
        Self { app_name }
    }

    #[process]
    pub async fn respond(&self) -> Response {
        Response::text(200, self.app_name.as_str())
    }
}

use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::config::app_name::AppName;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/health", server = "public")]
pub struct GetHealth {
    app_name: Arc<AppName>,
}

impl GetHealth {
    #[must_use]
    #[constructor]
    pub fn create(app_name: Arc<AppName>) -> Self {
        Self { app_name }
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self) -> anyhow::Result<Response> {
        Ok(Response::text(200, self.app_name.as_str()))
    }
}

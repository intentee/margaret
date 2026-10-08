use std::sync::Arc;

use tokio::task;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::diamond::retry_supervisor::RetrySupervisor;
use crate::diamond::session_validator::SessionValidator;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/diamond", server = "public")]
pub struct GetDiamond {
    _retry: Arc<RetrySupervisor>,
    _validator: Arc<SessionValidator>,
}

impl GetDiamond {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        retry: Arc<RetrySupervisor>,
        validator: Arc<SessionValidator>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            _retry: retry,
            _validator: validator,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(&self) -> anyhow::Result<Response> {
        task::yield_now().await;

        Ok(Response::text(200, "diamond"))
    }
}

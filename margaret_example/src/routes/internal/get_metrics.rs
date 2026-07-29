use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::metrics::Metrics;

#[singleton]
#[responds_to_http(method = "get", path = "/metrics", server = "internal")]
pub struct GetMetrics {
    metrics: Arc<Metrics>,
}

impl GetMetrics {
    #[constructor]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn create(metrics: Arc<Metrics>) -> anyhow::Result<Self> {
        Ok(Self { metrics })
    }

    #[process]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn respond(&self) -> anyhow::Result<Response> {
        Ok(Response::text(
            200,
            format!("sweeps={}", self.metrics.sweeps()),
        ))
    }
}

use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::metrics::Metrics;

#[singleton]
#[responds_to_http(method = "get", path = "/metrics", server = "internal")]
pub struct GetMetrics {
    metrics: Arc<Metrics>,
}

impl GetMetrics {
    #[constructor]
    #[must_use]
    pub fn create(metrics: Arc<Metrics>) -> Self {
        Self { metrics }
    }

    #[process]
    pub async fn respond(&self) -> Response {
        Response::text(200, format!("sweeps={}", self.metrics.sweeps()))
    }
}

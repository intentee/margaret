use std::error::Error;

use async_trait::async_trait;

use margaret_http::bind_route_parameter::bind_route_parameter;
use margaret_route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

struct FailingBinder;

#[async_trait]
impl HttpRouteParameterBinder for FailingBinder {
    type Model = u32;

    async fn bind(&self, _value: String) -> anyhow::Result<RouteParameterBindingOutcome<u32>> {
        anyhow::bail!("database unavailable")
    }
}

#[tokio::test]
async fn preserves_a_binder_failure_as_a_system_error() {
    let error = bind_route_parameter("42".to_string(), "number", &FailingBinder)
        .await
        .err()
        .expect("the binder fails");

    assert_eq!(
        error.to_string(),
        "the binder for route parameter 'number' failed: database unavailable"
    );
    assert_eq!(
        Error::source(&error).map(ToString::to_string),
        Some("database unavailable".to_string())
    );
}

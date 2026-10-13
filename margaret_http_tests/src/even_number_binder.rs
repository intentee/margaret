use async_trait::async_trait;

use margaret_route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

pub struct EvenNumberBinder;

#[async_trait]
impl HttpRouteParameterBinder for EvenNumberBinder {
    type Model = u32;

    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<u32>> {
        Ok(match value.parse::<u32>() {
            Ok(number) if number.is_multiple_of(2) => RouteParameterBindingOutcome::Bound(number),
            Ok(_) | Err(_) => RouteParameterBindingOutcome::NotFound,
        })
    }
}

use async_trait::async_trait;

use crate::route_parameter_binding_outcome::RouteParameterBindingOutcome;

#[async_trait]
pub trait HttpRouteParameterBinder {
    type Model;

    async fn bind(
        &self,
        value: String,
    ) -> anyhow::Result<RouteParameterBindingOutcome<Self::Model>>;
}

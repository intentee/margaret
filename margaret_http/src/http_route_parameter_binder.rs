use async_trait::async_trait;

use crate::route_parameter_outcome::RouteParameterOutcome;

#[async_trait]
pub trait HttpRouteParameterBinder {
    type Model;

    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterOutcome<Self::Model>>;
}

use anyhow::Result;
use async_trait::async_trait;
use margaret::framework::http::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret::framework::http::route_parameter_binding_outcome::RouteParameterBindingOutcome;
use margaret::framework::macros::provides_route_parameter;
use margaret::framework::macros::singleton;

use super::project::Project;

#[singleton]
#[provides_route_parameter]
pub struct ProjectStore;

#[async_trait]
impl HttpRouteParameterBinder for ProjectStore {
    type Model = Project;

    async fn bind(&self, value: String) -> Result<RouteParameterBindingOutcome<Self::Model>> {
        Ok(RouteParameterBindingOutcome::Bound(Project { name: value }))
    }
}

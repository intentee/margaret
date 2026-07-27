use async_trait::async_trait;
use failures::Result;
use margaret::framework::http::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret::framework::http::route_parameter_binding_outcome::RouteParameterBindingOutcome;
use margaret::framework::macros::provides_route_parameter;
use margaret::framework::macros::singleton;

use super::user::User;

#[singleton]
#[provides_route_parameter]
pub struct UserBinder;

#[async_trait]
impl HttpRouteParameterBinder for UserBinder {
    type Model = User;

    async fn bind(&self, value: String) -> Result<RouteParameterBindingOutcome<Self::Model>> {
        Ok(RouteParameterBindingOutcome::Bound(User { name: value }))
    }
}

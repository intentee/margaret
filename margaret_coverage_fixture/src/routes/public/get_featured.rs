use margaret::framework::http::forward::Forward;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::url_path::path_parameter_value::PathParameterValue;

use crate::margaret::routes::Routes;
use crate::margaret::routes::forward_targets::PublicForwardTarget;
use crate::margaret::routes::props::GetBookRouteProps;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/featured", server = "public")]
pub struct GetFeatured;

impl GetFeatured {
    /// # Errors
    ///
    /// Returns the rejection of a featured book value that cannot fill its route parameter.
    #[process]
    pub fn respond(&self, routes: &Routes) -> anyhow::Result<Forward<PublicForwardTarget>> {
        PathParameterValue::new("1".to_string())
            .and_then(|number| {
                PathParameterValue::new("fiction".to_string()).and_then(|shelf| {
                    PathParameterValue::new("Dune".to_string()).map(|title| {
                        routes.public.get_book.forward_to(GetBookRouteProps {
                            shelf,
                            number,
                            title,
                        })
                    })
                })
            })
            .map_err(anyhow::Error::from)
    }
}

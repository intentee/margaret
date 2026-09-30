use margaret_route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_route_parameter_binding::route_parameter_binding_error::RouteParameterBindingError;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

use crate::request::Request;

/// # Errors
///
/// Returns `RouteParameterBindingError::RouteParameterBinder`.
pub async fn require_bound_route_parameter<Binder>(
    request: &Request,
    name: &'static str,
    binder: &Binder,
) -> Result<RouteParameterBindingOutcome<Binder::Model>, RouteParameterBindingError>
where
    Binder: HttpRouteParameterBinder,
{
    match request.path_param(name) {
        Some(value) => binder.bind(value.to_string()).await.map_err(|source| {
            RouteParameterBindingError::RouteParameterBinder {
                parameter: name,
                source,
            }
        }),
        None => Ok(RouteParameterBindingOutcome::NotFound),
    }
}

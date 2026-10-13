use margaret_route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_route_parameter_binding::route_parameter_binding_error::RouteParameterBindingError;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

/// # Errors
///
/// Returns `RouteParameterBindingError::RouteParameterBinder`.
pub async fn bind_route_parameter<Binder>(
    value: String,
    name: &'static str,
    binder: &Binder,
) -> Result<RouteParameterBindingOutcome<Binder::Model>, RouteParameterBindingError>
where
    Binder: HttpRouteParameterBinder,
{
    binder
        .bind(value)
        .await
        .map_err(|source| RouteParameterBindingError::RouteParameterBinder {
            parameter: name,
            source,
        })
}

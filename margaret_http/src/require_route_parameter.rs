use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

use crate::request::Request;
use crate::requirement::Requirement;
use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

fn route_parameter_value<Value>(value: String) -> RouteParameterBindingOutcome<Value>
where
    Value: TryFrom<String>,
{
    match Value::try_from(value) {
        Ok(value) => RouteParameterBindingOutcome::Bound(value),
        Err(_) => RouteParameterBindingOutcome::NotFound,
    }
}

#[must_use]
pub fn require_route_parameter<Value>(request: &Request, name: &str) -> Requirement<Value>
where
    Value: TryFrom<String>,
{
    match request.path_param(name) {
        Some(value) => match route_parameter_value(value.to_string()) {
            RouteParameterBindingOutcome::Bound(value) => Requirement::Met(value),
            RouteParameterBindingOutcome::NotFound => {
                Requirement::Unmet(ResponseContinuation::from(Response::not_found()))
            }
        },
        None => Requirement::Unmet(ResponseContinuation::from(Response::not_found())),
    }
}

use http::Method;

use margaret_http::require_bound_route_parameter::require_bound_route_parameter;
use margaret_http_tests::even_number_binder::EvenNumberBinder;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

#[tokio::test]
async fn reports_an_absent_bound_route_parameter_as_not_found() {
    let request = FixtureRequest::new(Method::GET, "/numbers").into_request();
    let outcome = require_bound_route_parameter(&request, "number", &EvenNumberBinder).await;

    assert!(matches!(
        outcome,
        Ok(RouteParameterBindingOutcome::NotFound)
    ));
}

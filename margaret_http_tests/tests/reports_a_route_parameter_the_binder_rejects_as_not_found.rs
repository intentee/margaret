use margaret_http::require_bound_route_parameter::require_bound_route_parameter;
use margaret_http_tests::even_number_binder::EvenNumberBinder;
use margaret_http_tests::numbered_request::numbered_request;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

#[tokio::test]
async fn reports_a_route_parameter_the_binder_rejects_as_not_found() {
    let outcome =
        require_bound_route_parameter(&numbered_request("7"), "number", &EvenNumberBinder).await;

    assert!(matches!(
        outcome,
        Ok(RouteParameterBindingOutcome::NotFound)
    ));
}

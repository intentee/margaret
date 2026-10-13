use margaret_http::bind_route_parameter::bind_route_parameter;
use margaret_http_tests::even_number_binder::EvenNumberBinder;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

#[tokio::test]
async fn reports_a_route_parameter_the_binder_rejects_as_not_found() {
    let outcome = bind_route_parameter("7".to_string(), "number", &EvenNumberBinder).await;

    assert!(matches!(
        outcome,
        Ok(RouteParameterBindingOutcome::NotFound)
    ));
}

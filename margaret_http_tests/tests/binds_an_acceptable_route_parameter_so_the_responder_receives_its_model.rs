use margaret_http::bind_route_parameter::bind_route_parameter;
use margaret_http_tests::even_number_binder::EvenNumberBinder;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

#[tokio::test]
async fn binds_an_acceptable_route_parameter_so_the_responder_receives_its_model() {
    let outcome = bind_route_parameter("42".to_string(), "number", &EvenNumberBinder).await;

    assert!(matches!(
        outcome,
        Ok(RouteParameterBindingOutcome::Bound(42))
    ));
}

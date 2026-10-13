use http::Method;

use margaret_http::requirement::Requirement;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::with_route_parameters::with_route_parameters;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_http_tests::met_route_parameters::met_route_parameters;

#[tokio::test]
async fn refuses_a_request_missing_a_declared_route_parameter_as_not_found() {
    let request = FixtureRequest::new(Method::GET, "/numbers").into_request();

    assert!(matches!(
        with_route_parameters(&request, ["number"], Requirement::Unmet, met_route_parameters)
        .await,
        Requirement::Unmet(ResponseContinuation::Done(response)) if response.status() == 404
    ));
}

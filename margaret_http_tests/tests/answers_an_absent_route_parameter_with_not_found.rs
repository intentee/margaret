use http::Method;

use margaret_http::require_route_parameter::require_route_parameter;
use margaret_http::requirement::Requirement;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_tests::fixture_request::FixtureRequest;

#[test]
fn answers_an_absent_route_parameter_with_not_found() {
    let request = FixtureRequest::new(Method::GET, "/articles").into_request();

    assert!(matches!(
        require_route_parameter::<String>(&request, "article"),
        Requirement::Unmet(ResponseContinuation::Done(response)) if response.status() == 404
    ));
}

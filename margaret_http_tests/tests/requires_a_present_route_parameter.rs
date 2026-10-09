use std::collections::HashMap;

use http::Method;

use margaret_http::request::Request;
use margaret_http::require_route_parameter::require_route_parameter;
use margaret_http::requirement::Requirement;
use margaret_http_tests::fixture_request::FixtureRequest;

#[test]
fn requires_a_present_route_parameter() {
    let request: Request = FixtureRequest::new(Method::GET, "/articles/42")
        .into_request()
        .with_path_params(HashMap::from([("article".to_string(), "42".to_string())]));

    assert!(matches!(
        require_route_parameter::<String>(&request, "article"),
        Requirement::Met(value) if value == "42"
    ));
}

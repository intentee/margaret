use std::collections::HashMap;

use http::Method;

use margaret_http::request::Request;
use margaret_http::require_route_parameter::require_route_parameter;
use margaret_http::requirement::Requirement;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_tests::fixture_request::FixtureRequest;

struct PublishedYear;

impl TryFrom<String> for PublishedYear {
    type Error = std::num::ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse::<u16>().map(|_| Self)
    }
}

#[test]
fn answers_a_route_parameter_its_value_type_rejects_with_not_found() {
    let request: Request = FixtureRequest::new(Method::GET, "/articles/42")
        .into_request()
        .with_path_params(HashMap::from([(
            "article".to_string(),
            "not-a-year".to_string(),
        )]));

    assert!(matches!(
        require_route_parameter::<PublishedYear>(&request, "article"),
        Requirement::Unmet(ResponseContinuation::Done(response)) if response.status() == 404
    ));
}

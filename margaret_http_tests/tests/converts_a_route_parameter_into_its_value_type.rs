use std::collections::HashMap;

use http::Method;

use margaret_http::request::Request;
use margaret_http::require_route_parameter::require_route_parameter;
use margaret_http::requirement::Requirement;
use margaret_http_tests::fixture_request::FixtureRequest;

#[derive(Debug, Eq, PartialEq)]
struct ArticleSlug(String);

impl From<String> for ArticleSlug {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[test]
fn converts_a_route_parameter_into_its_value_type() {
    let request: Request = FixtureRequest::new(Method::GET, "/articles/42")
        .into_request()
        .with_path_params(HashMap::from([(
            "article".to_string(),
            "shipping-margaret".to_string(),
        )]));

    assert!(matches!(
        require_route_parameter::<ArticleSlug>(&request, "article"),
        Requirement::Met(ArticleSlug(slug)) if slug == "shipping-margaret"
    ));
}

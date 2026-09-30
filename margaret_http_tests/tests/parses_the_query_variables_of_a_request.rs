use http::Method;

use margaret_http_tests::fixture_request::FixtureRequest;

#[test]
fn parses_the_query_variables_of_a_request() {
    let request = FixtureRequest::new(Method::GET, "/search?term=rust&page=2").into_request();

    assert_eq!(
        request.inputs.query.get("term").map(String::as_str),
        Some("rust")
    );
    assert_eq!(request.inputs.server.query_string(), "term=rust&page=2");
}

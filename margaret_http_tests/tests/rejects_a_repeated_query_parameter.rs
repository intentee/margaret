use http::Method;

use margaret_http::request_rejection::RequestRejection;
use margaret_http_tests::fixture_request::FixtureRequest;

#[test]
fn rejects_a_repeated_query_parameter() {
    assert!(matches!(
        FixtureRequest::new(Method::GET, "/search?id=1&id=2").into_result(),
        Err(RequestRejection::DuplicateQueryParameter { name }) if name == "id"
    ));
}

use http::Method;

use margaret_http::request_rejection::RequestRejection;
use margaret_http_tests::fixture_request::FixtureRequest;

#[test]
fn rejects_an_ambiguous_request_path_so_dot_segments_cannot_escape_a_route() {
    assert!(matches!(
        FixtureRequest::new(Method::GET, "/a/../b").into_result(),
        Err(RequestRejection::DotSegmentInPath { .. })
    ));
}

use http::Method;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::body_rejection::BodyRejection;
use margaret_http::read_json_value::read_json_value;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::fixture_request::FixtureRequest;

#[tokio::test]
async fn rejects_a_json_body_without_a_content_type() {
    let request = FixtureRequest::new(Method::POST, "/content").into_request();

    assert!(matches!(
        read_json_value(&request, fixture_body(b"{}"), BodyLimit::new(64)).await,
        BodyReading::Rejected(BodyRejection::MissingContentType)
    ));
}

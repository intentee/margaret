use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::body_rejection::BodyRejection;
use margaret_http::request_body_stream::RequestBodyStream;
use margaret_http_tests::fixture_body::fixture_body;

#[test]
fn refuses_a_raw_body_whose_length_exceeds_its_limit_before_reading_it() {
    assert!(matches!(
        RequestBodyStream::open(fixture_body(b"0123456789"), BodyLimit::new(4)),
        BodyReading::Rejected(BodyRejection::PayloadTooLarge { limit: 4 })
    ));
}

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::body_rejection::BodyRejection;
use margaret_http::read_json_value::read_json_value;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn rejects_a_json_body_over_its_limit() {
    let request = content_type_request("application/json", UploadConfig::Disabled);

    assert!(matches!(
        read_json_value(&request, fixture_body(b"[1,2,3,4,5,6]"), BodyLimit::new(4)).await,
        BodyReading::Rejected(BodyRejection::PayloadTooLarge { limit: 4 })
    ));
}

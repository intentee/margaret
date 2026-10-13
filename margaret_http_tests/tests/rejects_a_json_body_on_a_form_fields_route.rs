use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::body_rejection::BodyRejection;
use margaret_http::read_form_fields::read_form_fields;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn rejects_a_json_body_on_a_form_fields_route() {
    let request = content_type_request("application/json", UploadConfig::Disabled);

    assert!(matches!(
        read_form_fields(&request, fixture_body(b"{}"), BodyLimit::new(64)).await,
        BodyReading::Rejected(BodyRejection::UnsupportedMediaType { media_type }) if media_type == mime::APPLICATION_JSON
    ));
}

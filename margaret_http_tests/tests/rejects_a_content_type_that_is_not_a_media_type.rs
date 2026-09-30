use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::body_rejection::BodyRejection;
use margaret_http::read_form_fields::read_form_fields;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn rejects_a_content_type_that_is_not_a_media_type() {
    let request = content_type_request("not/a/media/type", UploadConfig::Disabled);

    assert!(matches!(
        read_form_fields(&request, fixture_body(b"a=1"), BodyLimit::new(64)).await,
        BodyReading::Rejected(BodyRejection::MalformedContentType { .. })
    ));
}

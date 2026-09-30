use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::body_rejection::BodyRejection;
use margaret_http::read_form_fields::read_form_fields;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::multipart_fields::MULTIPART_FIELDS;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn rejects_a_multipart_form_without_a_boundary() {
    let request = content_type_request("multipart/form-data", UploadConfig::Disabled);

    assert!(matches!(
        read_form_fields(
            &request,
            fixture_body(MULTIPART_FIELDS),
            BodyLimit::new(1024)
        )
        .await,
        BodyReading::Rejected(BodyRejection::MissingMultipartBoundary)
    ));
}

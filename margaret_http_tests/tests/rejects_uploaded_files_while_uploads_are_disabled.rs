use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::body_rejection::BodyRejection;
use margaret_http::read_uploaded_files::read_uploaded_files;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::multipart_content_type::MULTIPART_CONTENT_TYPE;
use margaret_http_tests::multipart_files::MULTIPART_FILES;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn rejects_uploaded_files_while_uploads_are_disabled() {
    let request = content_type_request(MULTIPART_CONTENT_TYPE, UploadConfig::Disabled);

    assert!(matches!(
        read_uploaded_files(
            &request,
            fixture_body(MULTIPART_FILES),
            BodyLimit::new(1024)
        )
        .await,
        Ok(BodyReading::Rejected(BodyRejection::UploadsDisabled))
    ));
}

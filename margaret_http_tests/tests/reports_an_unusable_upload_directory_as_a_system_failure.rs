use margaret_http::body_limit::BodyLimit;
use margaret_http::read_uploaded_files::read_uploaded_files;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::multipart_content_type::MULTIPART_CONTENT_TYPE;
use margaret_http_tests::multipart_files::MULTIPART_FILES;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_http_uploaded_file::uploaded_file_error::UploadedFileError;

#[tokio::test]
async fn reports_an_unusable_upload_directory_as_a_system_failure() {
    let request = content_type_request(
        MULTIPART_CONTENT_TYPE,
        UploadConfig::enabled("/margaret-nonexistent-upload-directory".into()),
    );

    assert!(matches!(
        read_uploaded_files(
            &request,
            fixture_body(MULTIPART_FILES),
            BodyLimit::new(1024)
        )
        .await,
        Err(UploadedFileError::UploadTempFile { .. })
    ));
}

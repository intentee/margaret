use margaret_http::body_limit::BodyLimit;
use margaret_http::read_multipart::read_multipart;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::multipart_content_type::MULTIPART_CONTENT_TYPE;
use margaret_http_tests::multipart_fields_and_files::MULTIPART_FIELDS_AND_FILES;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_http_uploaded_file::uploaded_file_error::UploadedFileError;

#[tokio::test]
async fn reports_an_unusable_upload_directory_while_reading_a_multipart_body() {
    let request = content_type_request(
        MULTIPART_CONTENT_TYPE,
        UploadConfig::enabled("/margaret-nonexistent-upload-directory".into()),
    );

    assert!(matches!(
        read_multipart(
            &request,
            fixture_body(MULTIPART_FIELDS_AND_FILES),
            BodyLimit::new(1024)
        )
        .await,
        Err(UploadedFileError::UploadTempFile { .. })
    ));
}

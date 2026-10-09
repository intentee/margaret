use tempfile::tempdir;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::body_rejection::BodyRejection;
use margaret_http::read_uploaded_files::read_uploaded_files;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::multipart_content_type::MULTIPART_CONTENT_TYPE;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn rejects_a_repeated_upload_field() {
    let directory = tempdir().expect("a temporary directory");
    let request = content_type_request(
        MULTIPART_CONTENT_TYPE,
        UploadConfig::enabled(directory.path().to_path_buf()),
    );

    assert!(matches!(
        read_uploaded_files(&request, fixture_body(b"--X\r\nContent-Disposition: form-data; name=\"f\"; filename=\"a\"\r\n\r\nA\r\n--X\r\nContent-Disposition: form-data; name=\"f\"; filename=\"b\"\r\n\r\nB\r\n--X--\r\n"), BodyLimit::new(1024)).await,
        Ok(BodyReading::Rejected(BodyRejection::DuplicateUploadField { name })) if name == "f"
    ));
}

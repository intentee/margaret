use http::Method;
use tempfile::tempdir;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::body_rejection::BodyRejection;
use margaret_http::read_uploaded_files::read_uploaded_files;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_http_tests::multipart_files::MULTIPART_FILES;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn rejects_an_upload_without_a_content_type() {
    let directory = tempdir().expect("a temporary directory");
    let mut fixture = FixtureRequest::new(Method::POST, "/upload");

    fixture.upload_config = UploadConfig::enabled(directory.path().to_path_buf());

    assert!(matches!(
        read_uploaded_files(
            &fixture.into_request(),
            fixture_body(MULTIPART_FILES),
            BodyLimit::new(1024)
        )
        .await,
        Ok(BodyReading::Rejected(BodyRejection::MissingContentType))
    ));
}

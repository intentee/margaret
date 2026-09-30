use tempfile::tempdir;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::body_rejection::BodyRejection;
use margaret_http::read_uploaded_files::read_uploaded_files;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn rejects_an_upload_with_a_malformed_content_type() {
    let directory = tempdir().expect("a temporary directory");
    let request = content_type_request(
        "not/a/media/type",
        UploadConfig::enabled(directory.path().to_path_buf()),
    );

    assert!(matches!(
        read_uploaded_files(&request, fixture_body(b"a=1"), BodyLimit::new(1024)).await,
        Ok(BodyReading::Rejected(
            BodyRejection::MalformedContentType { .. }
        ))
    ));
}

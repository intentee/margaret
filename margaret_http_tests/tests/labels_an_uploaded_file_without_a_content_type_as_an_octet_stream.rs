use tempfile::tempdir;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::read_uploaded_files::read_uploaded_files;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::multipart_content_type::MULTIPART_CONTENT_TYPE;
use margaret_http_tests::multipart_files::MULTIPART_FILES;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn labels_an_uploaded_file_without_a_content_type_as_an_octet_stream() {
    let directory = tempdir().expect("a temporary directory");
    let request = content_type_request(
        MULTIPART_CONTENT_TYPE,
        UploadConfig::enabled(directory.path().to_path_buf()),
    );

    let Ok(BodyReading::Read(files)) = read_uploaded_files(
        &request,
        fixture_body(MULTIPART_FILES),
        BodyLimit::new(1024),
    )
    .await
    else {
        panic!("the uploaded files are read");
    };

    assert_eq!(
        files
            .get("raw")
            .expect("the raw file is uploaded")
            .content_type(),
        "application/octet-stream"
    );
}

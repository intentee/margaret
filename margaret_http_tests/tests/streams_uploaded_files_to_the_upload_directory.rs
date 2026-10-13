use std::fs;

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
async fn streams_uploaded_files_to_the_upload_directory() {
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
    let avatar = files.get("avatar").expect("the avatar is uploaded");

    assert_eq!(avatar.file_name(), "face.png");
    assert_eq!(avatar.content_type(), "image/png");
    assert_eq!(avatar.size(), 3);
    assert_eq!(avatar.path().parent(), Some(directory.path()));
    assert_eq!(
        fs::read(avatar.path()).expect("the upload is readable"),
        b"PNG"
    );
    assert_eq!(
        fs::read(files.get("raw").expect("the raw file is uploaded").path())
            .expect("the upload is readable"),
        b"DATA"
    );
}

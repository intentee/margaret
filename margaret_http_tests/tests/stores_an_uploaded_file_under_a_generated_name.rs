use std::ffi::OsStr;

use tempfile::tempdir;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::read_uploaded_files::read_uploaded_files;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::multipart_content_type::MULTIPART_CONTENT_TYPE;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn stores_an_uploaded_file_under_a_generated_name() {
    let directory = tempdir().expect("a temporary directory");
    let request = content_type_request(
        MULTIPART_CONTENT_TYPE,
        UploadConfig::enabled(directory.path().to_path_buf()),
    );

    let Ok(BodyReading::Read(files)) = read_uploaded_files(
        &request,
        fixture_body(b"--X\r\nContent-Disposition: form-data; name=\"avatar\"; filename=\"../../escape.png\"\r\n\r\nPNG\r\n--X--\r\n"),
        BodyLimit::new(1024),
    )
    .await
    else {
        panic!("the uploaded file is read");
    };
    let avatar = files.get("avatar").expect("the avatar is uploaded");

    assert_eq!(avatar.file_name(), "../../escape.png");
    assert_eq!(avatar.path().parent(), Some(directory.path()));
    assert_ne!(avatar.path().file_name(), Some(OsStr::new("escape.png")));
}

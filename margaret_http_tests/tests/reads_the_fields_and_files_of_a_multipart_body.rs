use tempfile::tempdir;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::multipart_content::MultipartContent;
use margaret_http::read_multipart::read_multipart;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::multipart_content_type::MULTIPART_CONTENT_TYPE;
use margaret_http_tests::multipart_fields_and_files::MULTIPART_FIELDS_AND_FILES;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_http_uploaded_file::uploaded_file::UploadedFile;

#[tokio::test]
async fn reads_the_fields_and_files_of_a_multipart_body() {
    let directory = tempdir().expect("a temporary directory");
    let request = content_type_request(
        MULTIPART_CONTENT_TYPE,
        UploadConfig::enabled(directory.path().to_path_buf()),
    );

    let Ok(BodyReading::Read(MultipartContent { fields, files })) = read_multipart(
        &request,
        fixture_body(MULTIPART_FIELDS_AND_FILES),
        BodyLimit::new(1024),
    )
    .await
    else {
        panic!("the multipart body is read");
    };

    assert_eq!(fields.get("title").map(String::as_str), Some("hello"));
    assert_eq!(
        files.get("avatar").map(UploadedFile::file_name),
        Some("face.png")
    );
}

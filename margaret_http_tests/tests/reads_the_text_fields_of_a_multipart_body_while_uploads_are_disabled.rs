use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::multipart_content::MultipartContent;
use margaret_http::read_multipart::read_multipart;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::multipart_content_type::MULTIPART_CONTENT_TYPE;
use margaret_http_tests::multipart_fields::MULTIPART_FIELDS;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn reads_the_text_fields_of_a_multipart_body_while_uploads_are_disabled() {
    let request = content_type_request(MULTIPART_CONTENT_TYPE, UploadConfig::Disabled);

    let Ok(BodyReading::Read(MultipartContent { fields, files })) = read_multipart(
        &request,
        fixture_body(MULTIPART_FIELDS),
        BodyLimit::new(1024),
    )
    .await
    else {
        panic!("the text fields are read without an upload directory");
    };

    assert_eq!(fields.get("title").map(String::as_str), Some("hello"));
    assert_eq!(fields.get("subtitle").map(String::as_str), Some("world"));
    assert!(files.get("avatar").is_none());
}

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::read_form_fields::read_form_fields;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::multipart_content_type::MULTIPART_CONTENT_TYPE;
use margaret_http_tests::multipart_fields::MULTIPART_FIELDS;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn reads_the_text_fields_of_a_multipart_form() {
    let request = content_type_request(MULTIPART_CONTENT_TYPE, UploadConfig::Disabled);

    let BodyReading::Read(fields) = read_form_fields(
        &request,
        fixture_body(MULTIPART_FIELDS),
        BodyLimit::new(1024),
    )
    .await
    else {
        panic!("a multipart form without files is read");
    };

    assert_eq!(fields.get("title").map(String::as_str), Some("hello"));
    assert_eq!(fields.get("subtitle").map(String::as_str), Some("world"));
}

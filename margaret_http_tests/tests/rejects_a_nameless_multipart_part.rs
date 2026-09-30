use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::body_rejection::BodyRejection;
use margaret_http::read_form_fields::read_form_fields;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_tests::multipart_content_type::MULTIPART_CONTENT_TYPE;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn rejects_a_nameless_multipart_part() {
    let request = content_type_request(MULTIPART_CONTENT_TYPE, UploadConfig::Disabled);

    assert!(matches!(
        read_form_fields(
            &request,
            fixture_body(b"--X\r\nContent-Disposition: form-data\r\n\r\nhello\r\n--X--\r\n"),
            BodyLimit::new(1024)
        )
        .await,
        BodyReading::Rejected(BodyRejection::NamelessMultipartField)
    ));
}

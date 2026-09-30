use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::read_form_fields::read_form_fields;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn reads_the_fields_of_an_urlencoded_form() {
    let request = content_type_request("application/x-www-form-urlencoded", UploadConfig::Disabled);

    let BodyReading::Read(fields) = read_form_fields(
        &request,
        fixture_body(b"username=margaret&note=a%20b"),
        BodyLimit::new(64),
    )
    .await
    else {
        panic!("an urlencoded form is read");
    };

    assert_eq!(fields.get("username").map(String::as_str), Some("margaret"));
    assert_eq!(fields.get("note").map(String::as_str), Some("a b"));
}

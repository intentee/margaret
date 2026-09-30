use serde_json::json;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::read_json_value::read_json_value;
use margaret_http_tests::content_type_request::content_type_request;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn reads_a_json_value() {
    let request = content_type_request("application/json; charset=utf-8", UploadConfig::Disabled);

    let BodyReading::Read(value) = read_json_value(
        &request,
        fixture_body(b"{\"title\":\"hello\"}"),
        BodyLimit::new(64),
    )
    .await
    else {
        panic!("a json body is read");
    };

    assert_eq!(value, json!({ "title": "hello" }));
}

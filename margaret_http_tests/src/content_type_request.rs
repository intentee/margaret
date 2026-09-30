use http::HeaderValue;
use http::Method;
use http::header::CONTENT_TYPE;

use margaret_http::request::Request;
use margaret_http_uploaded_file::upload_config::UploadConfig;

use crate::fixture_request::FixtureRequest;

#[must_use]
pub fn content_type_request(content_type: &'static str, upload_config: UploadConfig) -> Request {
    let mut fixture = FixtureRequest::new(Method::POST, "/content");

    fixture
        .headers
        .insert(CONTENT_TYPE, HeaderValue::from_static(content_type));
    fixture.upload_config = upload_config;
    fixture.into_request()
}

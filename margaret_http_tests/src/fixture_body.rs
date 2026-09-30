use bytes::Bytes;
use http_body_util::Full;

use margaret_http::request_body::RequestBody;

#[must_use]
pub fn fixture_body(content: &'static [u8]) -> RequestBody {
    RequestBody::new(Full::new(Bytes::from_static(content)))
}

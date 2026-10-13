use bytes::Bytes;
use http_body_util::Full;

use margaret_http::request_body::RequestBody;

#[must_use]
pub fn owned_body(content: String) -> RequestBody {
    RequestBody::new(Full::new(Bytes::from(content)))
}

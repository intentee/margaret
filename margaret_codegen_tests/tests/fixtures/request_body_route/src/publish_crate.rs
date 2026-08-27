use margaret::framework::http::bytes::Bytes;
use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

#[singleton]
#[responds_to_http(method = "put", path = "/api/v1/crates/new", server = "public")]
pub struct PublishCrate;

impl PublishCrate {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, #[request_body] upload: &Bytes) -> anyhow::Result<Response> {
        Ok(Response::text(201, format!("{}", upload.len())))
    }
}

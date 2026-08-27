use margaret::framework::http::bytes::Bytes;
use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

#[singleton]
#[responds_to_http(method = "post", path = "/articles/archive", server = "public")]
pub struct PostArticleArchive;

impl PostArticleArchive {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, #[request_body] archive: &Bytes) -> anyhow::Result<Response> {
        Ok(Response::text(
            201,
            format!("stored an archive of {} bytes", archive.len()),
        ))
    }
}

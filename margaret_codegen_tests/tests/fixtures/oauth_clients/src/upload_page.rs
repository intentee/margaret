use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::artifact_uploader::ArtifactUploader;

#[singleton]
#[responds_to_http(method = "get", path = "/uploads", server = "public")]
pub struct UploadPage;

impl UploadPage {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] uploader: ArtifactUploader,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(200, uploader.repository))
    }
}

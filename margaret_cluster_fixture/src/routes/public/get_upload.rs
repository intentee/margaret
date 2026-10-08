use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::models::upload::Upload;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_upload",
    path = "/uploads/{upload}",
    server = "public",
)]
pub struct GetUpload;

impl GetUpload {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[route_parameter(from = "upload")] Upload { content, .. }: Upload,
    ) -> anyhow::Result<Response> {
        Ok(Response::bytes(200, "application/octet-stream", content))
    }
}

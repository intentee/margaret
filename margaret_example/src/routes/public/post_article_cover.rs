use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::http_uploaded_file::uploaded_files::UploadedFiles;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::models::article::Article;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = RouteMethod::Post,
    path = "/articles/{article}/cover",
    server = "public"
)]
pub struct PostArticleCover;

impl PostArticleCover {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        request: &Request,
        #[route_parameter(from = "article")] Article { title, .. }: Article,
        files: UploadedFiles,
    ) -> anyhow::Result<Response> {
        Ok({
            let Some(cover) = files.get("cover") else {
                return Ok(Response::text(422, "a \"cover\" file upload is required"));
            };

            Response::text(
                201,
                format!(
                    "stored {} byte {} cover for \"{title}\" from {}",
                    cover.size(),
                    cover.content_type(),
                    request.inputs.server.remote_addr(),
                ),
            )
        })
    }
}

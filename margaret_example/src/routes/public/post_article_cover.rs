use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::models::article::Article;

#[singleton]
#[responds_to_http(method = "post", path = "/articles/{article}/cover", server = "public")]
pub struct PostArticleCover;

impl PostArticleCover {
    #[process]
    pub fn respond(
        &self,
        request: &Request,
        #[route_parameter(from = "article")] Article { title, .. }: Article,
    ) -> anyhow::Result<Response> {
        Ok({
            let Some(cover) = request.inputs.files.get("cover") else {
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

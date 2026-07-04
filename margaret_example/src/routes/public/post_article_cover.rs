use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::models::article::Article;

#[singleton]
#[responds_to_http(method = "post", path = "/articles/{article}/cover", server = "public")]
pub struct PostArticleCover;

impl PostArticleCover {
    #[process]
    pub async fn respond(
        &self,
        request: &Request,
        #[route_parameter(from = "article")] Article { title, .. }: Article,
    ) -> Response {
        let Some(cover) = request.inputs.files.get("cover") else {
            return Response::text(422, "a \"cover\" file upload is required");
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
    }
}

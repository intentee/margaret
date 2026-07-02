use margaret_http::response::Response;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::models::article::Article;

#[singleton]
#[responds_to_http(method = Get, path = "/articles/{article}", server = "public")]
pub struct GetArticle;

impl GetArticle {
    #[responder]
    pub async fn respond(
        &self,
        #[route_parameter(from = "article")] Article {
            title,
            body,
            created_at,
            ..
        }: Article,
    ) -> Response {
        Response::text(200, format!("\"{title}\" (posted at {created_at}): {body}"))
    }
}

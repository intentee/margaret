use margaret_http::response::Response;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::models::article::Article;

#[singleton]
#[responds_to_http(method = Get, name = "get_article", path = "/articles/{article}", server = "public")]
pub struct GetArticle;

impl GetArticle {
    #[process]
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

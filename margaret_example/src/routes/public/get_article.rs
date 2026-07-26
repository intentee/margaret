use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::models::article::Article;

#[singleton]
#[responds_to_http(
    method = "get",
    name = "get_article",
    path = "/articles/{article}",
    server = "public"
)]
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
    ) -> anyhow::Result<Response> {
        Ok(Response::text(
            200,
            format!("\"{title}\" (posted at {created_at}): {body}"),
        ))
    }
}

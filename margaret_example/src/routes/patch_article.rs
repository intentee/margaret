use margaret_http::response::Response;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::models::article::Article;

#[singleton]
#[responds_to_http(method = Patch, path = "/articles/{article}")]
pub struct PatchArticle;

impl PatchArticle {
    #[responder]
    pub async fn respond(
        &self,
        #[route_parameter(intent = CrudAction::Update)] article: Article,
    ) -> Response {
        Response::text(200, format!("updated \"{}\"", article.title))
    }
}

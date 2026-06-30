use margaret_http::response::Response;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::models::article::Article;

#[singleton]
#[responds_to_http(method = Delete, path = "/articles/{article}")]
pub struct DeleteArticle;

impl DeleteArticle {
    #[responder]
    pub async fn respond(
        &self,
        #[route_parameter(intent = CrudAction::Delete)] article: Article,
    ) -> Response {
        Response::text(200, format!("deleted \"{}\"", article.title))
    }
}

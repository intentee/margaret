use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::models::article::Article;
use crate::repositories::article_repository::ArticleRepository;

#[singleton]
#[responds_to_http(method = Delete, path = "/articles/{article}", server = "public")]
pub struct DeleteArticle {
    articles: Arc<ArticleRepository>,
}

impl DeleteArticle {
    #[constructor]
    pub fn create(articles: Arc<ArticleRepository>) -> Self {
        Self { articles }
    }

    #[responder]
    pub async fn respond(
        &self,
        #[route_parameter(intent = CrudAction::Delete)] article: Article,
    ) -> Response {
        self.articles.remove(&article.id);

        Response::text(200, format!("deleted \"{}\"", article.title))
    }
}

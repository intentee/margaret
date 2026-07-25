use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::models::article::Article;
use crate::stores::article_store::ArticleStore;
use crate::stores::respond_to_article_store_error::respond_to_article_store_error;

#[singleton]
#[responds_to_http(method = "delete", path = "/articles/{article}", server = "public")]
pub struct DeleteArticle {
    articles: Arc<ArticleStore>,
}

impl DeleteArticle {
    #[constructor]
    #[must_use]
    pub fn create(articles: Arc<ArticleStore>) -> Self {
        Self { articles }
    }

    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "article")] Article { id, title, .. }: Article,
    ) -> Response {
        if let Err(error) = self.articles.remove(id).await {
            return respond_to_article_store_error(&error);
        }

        Response::text(200, format!("deleted \"{title}\""))
    }
}

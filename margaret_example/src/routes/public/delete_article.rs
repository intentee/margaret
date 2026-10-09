use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::models::article::Article;
use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(method = RouteMethod::Delete, path = "/articles/{article}", server = "public")]
pub struct DeleteArticle {
    articles: Arc<ArticleStore>,
}

impl DeleteArticle {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(articles: Arc<ArticleStore>) -> anyhow::Result<Self> {
        Ok(Self { articles })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "article")] Article { id, title, .. }: Article,
    ) -> anyhow::Result<Response> {
        Ok({
            self.articles.remove(id).await?;

            Response::text(200, format!("deleted \"{title}\""))
        })
    }
}

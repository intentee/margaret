use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::models::article::Article;
use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(access = margaret::framework::http::public_access::PublicAccess, method = "delete", path = "/articles/{article}", server = "public")]
pub struct DeleteArticle {
    articles: Arc<ArticleStore>,
}

impl DeleteArticle {
    #[constructor]
    pub fn create(articles: Arc<ArticleStore>) -> anyhow::Result<Self> {
        Ok(Self { articles })
    }

    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "article")] Article { id, title, .. }: Article,
    ) -> anyhow::Result<Response> {
        Ok({
            self.articles.remove(id);

            Response::text(200, format!("deleted \"{title}\""))
        })
    }
}

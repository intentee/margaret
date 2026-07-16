use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::forms::patch_article_form::PatchArticleForm;
use crate::models::article::Article;
use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(method = "patch", path = "/articles/{article}", server = "public")]
pub struct PatchArticle {
    articles: Arc<ArticleStore>,
}

impl PatchArticle {
    #[constructor]
    pub fn create(articles: Arc<ArticleStore>) -> Self {
        Self { articles }
    }

    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "article")] Article {
            id,
            title: current_title,
            author_id,
            body: current_body,
            published,
            created_at,
        }: Article,
        #[form_request(from = Form)] PatchArticleForm { title, body }: PatchArticleForm,
    ) -> Response {
        let title = title.unwrap_or(current_title);
        let body = body.unwrap_or(current_body);

        self.articles.save(Article {
            id,
            title: title.clone(),
            author_id,
            body,
            published,
            created_at,
        });

        Response::text(200, format!("updated \"{title}\""))
    }
}

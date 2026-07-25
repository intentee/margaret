use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

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
    #[must_use]
    pub fn create(articles: Arc<ArticleStore>) -> Self {
        Self { articles }
    }

    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "article")] Article {
            id,
            title: current_title,
            body: current_body,
            cover,
            published,
            status,
            created_at,
            author,
        }: Article,
        #[form_request(from = Form)] PatchArticleForm { title, body }: PatchArticleForm,
    ) -> Response {
        let title = title.unwrap_or(current_title);
        let body = body.unwrap_or(current_body);

        if let Err(error) = self
            .articles
            .save(Article {
                id,
                title: title.clone(),
                body,
                cover,
                published,
                status,
                created_at,
                author,
            })
            .await
        {
            return Response::text(500, error.to_string());
        }

        Response::text(200, format!("updated \"{title}\""))
    }
}

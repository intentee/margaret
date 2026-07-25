use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::forms::post_article_form::PostArticleForm;
use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(
    method = "post",
    name = "post_article",
    path = "/articles",
    server = "public"
)]
pub struct PostArticle {
    articles: Arc<ArticleStore>,
}

impl PostArticle {
    #[constructor]
    #[must_use]
    pub fn create(articles: Arc<ArticleStore>) -> Self {
        Self { articles }
    }

    #[process]
    pub async fn respond(
        &self,
        #[form_request(from = Form)] PostArticleForm {
            title,
            body,
            author_id,
        }: PostArticleForm,
    ) -> Response {
        match self.articles.insert(title, body, author_id).await {
            Ok(article) => Response::text(201, format!("created \"{}\"", article.title)),
            Err(error) => Response::text(500, error.to_string()),
        }
    }
}

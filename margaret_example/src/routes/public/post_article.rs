use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

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
        match self.articles.insert(title, body, author_id) {
            Ok(article) => Response::text(201, format!("created \"{}\"", article.title)),
            Err(error) => Response::text(404, error.to_string()),
        }
    }
}

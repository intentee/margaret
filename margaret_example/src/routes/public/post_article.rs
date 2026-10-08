use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::forms::post_article_form::PostArticleForm;
use crate::stores::article_insertion::ArticleInsertion;
use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = RouteMethod::Post,
    name = "post_article",
    path = "/articles",
    server = "public"
)]
pub struct PostArticle {
    articles: Arc<ArticleStore>,
}

impl PostArticle {
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
        #[form_request(from = RequestInput::Form)] PostArticleForm {
            title,
            body,
            author_id,
        }: PostArticleForm,
    ) -> anyhow::Result<Response> {
        Ok({
            match self.articles.insert(title, body, author_id).await? {
                ArticleInsertion::AuthorNotFound(refusal) => {
                    Response::text(500, refusal.to_string())
                }
                ArticleInsertion::Inserted(article) => {
                    Response::text(201, format!("created \"{}\"", article.title))
                }
            }
        })
    }
}

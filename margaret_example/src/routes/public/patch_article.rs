use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::forms::patch_article_form::PatchArticleForm;
use crate::models::article::Article;
use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = RouteMethod::Patch,
    path = "/articles/{article}",
    server = "public"
)]
pub struct PatchArticle {
    articles: Arc<ArticleStore>,
}

impl PatchArticle {
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
        #[route_parameter(from = "article")] Article {
            id,
            title: current_title,
            body: current_body,
            cover,
            published,
            price,
            reading_minutes,
            status,
            created_at,
            author,
        }: Article,
        #[form_request(from = RequestInput::Form)]
        PatchArticleForm { title, body }: PatchArticleForm,
    ) -> anyhow::Result<Response> {
        Ok({
            let title = title.unwrap_or(current_title);
            let body = body.unwrap_or(current_body);

            self.articles
                .save(&Article {
                    id,
                    title: title.clone(),
                    body,
                    cover,
                    published,
                    price,
                    reading_minutes,
                    status,
                    created_at,
                    author,
                })
                .await?;

            Response::text(200, format!("updated \"{title}\""))
        })
    }
}

use std::sync::Arc;

use margaret::framework::active_record::creation::Creation;
use margaret::framework::database::database::Database;
use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::author_not_found::AuthorNotFound;
use crate::forms::post_article_form::PostArticleForm;
use crate::models::article::Article;
use crate::system_clock::SystemClock;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = RouteMethod::Post,
    name = "post_article",
    path = "/articles",
    server = "public"
)]
pub struct PostArticle {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
}

impl PostArticle {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { clock, database })
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
        Ok(
            match Article::draft(&self.database, title, body, author_id, self.clock.now()).await? {
                Creation::Created(article) => {
                    Response::text(201, format!("created \"{}\"", article.title))
                }
                Creation::Refused => Response::text(500, AuthorNotFound { author_id }.to_string()),
            },
        )
    }
}

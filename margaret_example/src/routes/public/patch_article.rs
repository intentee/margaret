use std::sync::Arc;

use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::saving::Saving;
use margaret::framework::database::database::Database;
use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::forms::patch_article_form::PatchArticleForm;
use crate::models::article::Article;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = RouteMethod::Patch,
    path = "/articles/{article}",
    server = "public"
)]
pub struct PatchArticle {
    database: Arc<Database>,
}

impl PatchArticle {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "article")] article: Article,
        #[form_request(from = RequestInput::Form)]
        PatchArticleForm { title, body }: PatchArticleForm,
    ) -> anyhow::Result<Response> {
        let patched = Article {
            title: title.unwrap_or(article.title),
            body: body.unwrap_or(article.body),
            ..article
        };

        Ok(match patched.save(self.database.as_ref()).await? {
            Saving::Saved => Response::text(200, format!("updated \"{}\"", patched.title)),
            Saving::Missing => Response::not_found(),
        })
    }
}

use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::validation::validation_result::ValidationResult;

use crate::forms::post_article_form::PostArticleForm;
use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = RouteMethod::Post,
    path = "/articles/import",
    server = "public"
)]
pub struct PostArticleImport {
    articles: Arc<ArticleStore>,
}

impl PostArticleImport {
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
    pub fn respond(
        &self,
        #[form_request(from = RequestInput::Json)] form: ValidationResult<PostArticleForm>,
    ) -> anyhow::Result<Response> {
        Ok({
            let PostArticleForm {
                title,
                body,
                author_id,
            } = match form {
                ValidationResult::Valid(form) => form,
                ValidationResult::Invalid(errors) => {
                    return Ok(Response::text(422, errors.to_string()));
                }
                ValidationResult::Malformed(malformation) => {
                    return Ok(Response::text(400, malformation.to_string()));
                }
            };

            match self.articles.insert(title, body, author_id) {
                Ok(article) => Response::text(201, format!("imported \"{}\"", article.title)),
                Err(error) => Response::text(500, error.to_string()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use uuid::Uuid;

    use margaret::framework::validation::validation_result::ValidationResult;

    use super::PostArticleImport;
    use crate::forms::post_article_form::PostArticleForm;
    use crate::stores::article_store::ArticleStore;
    use crate::system_clock::SystemClock;

    #[tokio::test]
    async fn responds_with_500_when_the_author_is_unknown() {
        let clock = SystemClock::create().expect("the clock is constructed");
        let store =
            ArticleStore::create(Arc::new(clock)).expect("the article store is constructed");
        let responder =
            PostArticleImport::create(Arc::new(store)).expect("the responder is constructed");
        let form = ValidationResult::Valid(PostArticleForm {
            title: "Title".to_string(),
            body: "Body".to_string(),
            author_id: Uuid::from_u128(999),
        });

        assert_eq!(
            responder
                .respond(form)
                .expect("the responder succeeds")
                .status(),
            500
        );
    }
}

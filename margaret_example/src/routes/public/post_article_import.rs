use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::validation::validation_result::ValidationResult;

use crate::forms::post_article_form::PostArticleForm;
use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(method = "post", path = "/articles/import", server = "public")]
pub struct PostArticleImport {
    articles: Arc<ArticleStore>,
}

impl PostArticleImport {
    #[constructor]
    #[must_use]
    pub fn create(articles: Arc<ArticleStore>) -> Self {
        Self { articles }
    }

    #[process]
    pub async fn respond(
        &self,
        #[form_request(from = Json)] form: ValidationResult<PostArticleForm>,
    ) -> Response {
        let PostArticleForm {
            title,
            body,
            author_id,
        } = match form {
            ValidationResult::Valid(form) => form,
            ValidationResult::Invalid(errors) => return Response::text(422, errors.to_string()),
            ValidationResult::Malformed(malformation) => {
                return Response::text(400, malformation.to_string());
            }
        };

        match self.articles.insert(title, body, author_id).await {
            Ok(article) => Response::text(201, format!("imported \"{}\"", article.title)),
            Err(error) => Response::text(500, error.to_string()),
        }
    }
}

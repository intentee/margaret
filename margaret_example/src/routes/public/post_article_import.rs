use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;
use margaret_validation::validation_result::ValidationResult;

use crate::forms::post_article_form::PostArticleForm;
use crate::stores::article_store::ArticleStore;

#[singleton]
#[responds_to_http(method = "post", path = "/articles/import", server = "public")]
pub struct PostArticleImport {
    articles: Arc<ArticleStore>,
}

impl PostArticleImport {
    #[constructor]
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

        match self.articles.insert(title, body, author_id) {
            Ok(article) => Response::text(201, format!("imported \"{}\"", article.title)),
            Err(error) => Response::text(500, error.to_string()),
        }
    }
}

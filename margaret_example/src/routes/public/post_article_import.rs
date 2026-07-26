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
    pub fn create(articles: Arc<ArticleStore>) -> Self {
        Self { articles }
    }

    #[process]
    pub async fn respond(
        &self,
        #[form_request(from = Json)] form: ValidationResult<PostArticleForm>,
    ) -> anyhow::Result<Response> {
        let PostArticleForm {
            title,
            body,
            author_id,
        } = match form {
            ValidationResult::Valid(form) => form,
            ValidationResult::Invalid(errors) => return Ok(Response::text(422, errors.to_string())),
            ValidationResult::Malformed(malformation) => {
                return Ok(Response::text(400, malformation.to_string()));
            }
        };

        let article = self.articles.insert(title, body, author_id)?;

        Ok(Response::text(201, format!("imported \"{}\"", article.title)))
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
    async fn reports_an_error_when_the_author_is_unknown() {
        let responder = PostArticleImport::create(Arc::new(ArticleStore::create(Arc::new(
            SystemClock::create(),
        ))));
        let form = ValidationResult::Valid(PostArticleForm {
            title: "Title".to_string(),
            body: "Body".to_string(),
            author_id: Uuid::from_u128(999),
        });

        assert!(responder.respond(form).await.is_err());
    }
}

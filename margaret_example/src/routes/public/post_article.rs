use std::sync::Arc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;
use margaret_validation::ValidationResult;
use margaret_validation::validate;

use crate::forms::post_article_form::PostArticleForm;
use crate::repositories::article_repository::ArticleRepository;

#[singleton]
#[responds_to_http(method = Post, path = "/articles", server = "public")]
pub struct PostArticle {
    articles: Arc<ArticleRepository>,
}

impl PostArticle {
    #[constructor]
    pub fn create(articles: Arc<ArticleRepository>) -> Self {
        Self { articles }
    }

    #[responder]
    pub async fn respond(&self, request: &Request) -> Response {
        let PostArticleForm {
            title,
            body,
            author_id,
        } = match validate::<PostArticleForm>(&request.inputs.form) {
            ValidationResult::Valid(form) => form,
            ValidationResult::Invalid(errors) => return Response::text(422, errors.to_string()),
        };

        let article = self.articles.insert(title, body, author_id);

        Response::text(201, format!("created \"{}\"", article.title))
    }
}

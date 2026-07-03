use std::sync::Arc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;
use margaret_validation::ValidationResult;
use margaret_validation::validate;

use crate::forms::patch_article_form::PatchArticleForm;
use crate::models::article::Article;
use crate::repositories::article_repository::ArticleRepository;

#[singleton]
#[responds_to_http(method = Patch, path = "/articles/{article}", server = "public")]
pub struct PatchArticle {
    articles: Arc<ArticleRepository>,
}

impl PatchArticle {
    #[constructor]
    pub fn create(articles: Arc<ArticleRepository>) -> Self {
        Self { articles }
    }

    #[responder]
    pub async fn respond(
        &self,
        request: &Request,
        #[route_parameter(from = "article")] Article {
            id,
            title: current_title,
            author_id,
            body: current_body,
            published,
            created_at,
        }: Article,
    ) -> Response {
        let PatchArticleForm { title, body } =
            match validate::<PatchArticleForm>(&request.inputs.form) {
                ValidationResult::Valid(form) => form,
                ValidationResult::Invalid(errors) => {
                    return Response::text(422, errors.to_string());
                }
            };

        let title = title.unwrap_or(current_title);
        let body = body.unwrap_or(current_body);

        self.articles.save(Article {
            id,
            title: title.clone(),
            author_id,
            body,
            published,
            created_at,
        });

        Response::text(200, format!("updated \"{title}\""))
    }
}

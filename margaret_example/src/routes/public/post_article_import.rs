use std::sync::Arc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::repositories::article_repository::ArticleRepository;

#[singleton]
#[responds_to_http(method = Post, path = "/articles/import", server = "public")]
pub struct PostArticleImport {
    articles: Arc<ArticleRepository>,
}

impl PostArticleImport {
    #[constructor]
    pub fn create(articles: Arc<ArticleRepository>) -> Self {
        Self { articles }
    }

    #[responder]
    pub async fn respond(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::text(415, "a JSON request body is required");
        };

        let (Some(title), Some(body), Some(author_id)) = (
            payload["title"].as_str(),
            payload["body"].as_str(),
            payload["author_id"].as_str(),
        ) else {
            return Response::text(422, "title, body, and author_id are required");
        };

        let article =
            self.articles
                .insert(title.to_string(), body.to_string(), author_id.to_string());

        Response::text(201, format!("imported \"{}\"", article.title))
    }
}

use std::sync::Arc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

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
        let (Some(title), Some(body), Some(author_id)) = (
            request.form("title"),
            request.form("body"),
            request.form("author_id"),
        ) else {
            return Response::text(422, "title, body, and author_id are required");
        };

        let article =
            self.articles
                .insert(title.to_string(), body.to_string(), author_id.to_string());

        Response::text(201, format!("created \"{}\"", article.title))
    }
}

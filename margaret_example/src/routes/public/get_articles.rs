use std::sync::Arc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::repositories::article_repository::ArticleRepository;

#[singleton]
#[responds_to_http(method = Get, path = "/articles", server = "public")]
pub struct GetArticles {
    articles: Arc<ArticleRepository>,
}

impl GetArticles {
    #[constructor]
    pub fn create(articles: Arc<ArticleRepository>) -> Self {
        Self { articles }
    }

    #[responder]
    pub async fn respond(&self, request: &Request) -> Response {
        let author = request.query("author");
        let titles = self
            .articles
            .all()
            .into_iter()
            .filter(|article| match author {
                Some(author) => article.author_id.as_str() == author,
                None => true,
            })
            .map(|article| article.title)
            .collect::<Vec<String>>()
            .join(", ");

        Response::text(200, titles)
    }
}

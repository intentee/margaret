use std::sync::Arc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

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
        }: Article,
    ) -> Response {
        let title = match request.form("title") {
            Some(title) => title,
            None => current_title,
        };
        let body = match request.form("body") {
            Some(body) => body,
            None => current_body,
        };

        self.articles.save(Article {
            id,
            title: title.clone(),
            author_id,
            body,
            published,
        });

        Response::text(200, format!("updated \"{title}\""))
    }
}

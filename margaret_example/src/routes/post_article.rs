use std::sync::Arc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;
use margaret_security::actor::Actor;

use crate::models::user::User;
use crate::repositories::article_repository::ArticleRepository;

#[singleton]
#[responds_to_http(method = Post, path = "/articles")]
pub struct PostArticle {
    articles: Arc<ArticleRepository>,
}

impl PostArticle {
    #[constructor]
    pub fn create(articles: Arc<ArticleRepository>) -> Self {
        Self { articles }
    }

    #[responder]
    pub async fn respond(
        &self,
        request: &Request,
        #[session_authenticated] author: User,
    ) -> Response {
        let (Some(title), Some(body)) = (request.form("title"), request.form("body")) else {
            return Response::text(422, "title and body are required");
        };

        let article = self
            .articles
            .insert(title, body, author.identifier().to_string());

        Response::text(201, format!("created \"{}\"", article.title))
    }
}

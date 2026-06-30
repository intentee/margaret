use std::sync::Arc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;
use margaret_security::crud_action::CrudAction;

use crate::action::Action;
use crate::margaret::security::Gatekeeper;
use crate::repositories::article_repository::ArticleRepository;

#[singleton]
#[responds_to_http(method = Get, path = "/articles")]
pub struct GetArticles {
    articles: Arc<ArticleRepository>,
    gatekeeper: Arc<Gatekeeper>,
}

impl GetArticles {
    #[constructor]
    pub fn create(articles: Arc<ArticleRepository>, gatekeeper: Arc<Gatekeeper>) -> Self {
        Self {
            articles,
            gatekeeper,
        }
    }

    #[responder]
    pub async fn respond(&self, request: &Request) -> Response {
        let context = self.gatekeeper.with_request(request).await;
        let manages_users = context.can(Action::ManageUsers).await;
        let articles = self.articles.all();
        let can_delete_all = context.can_crud_all(&articles, CrudAction::Delete).await;

        Response::text(
            200,
            format!("manage_users={manages_users};delete_all={can_delete_all}"),
        )
    }
}

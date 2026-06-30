use std::sync::Arc;

use margaret_example_macros::traced;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::can;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::margaret::security::Gatekeeper;
use crate::repositories::article_repository::ArticleRepository;
use crate::repositories::user_repository::UserRepository;

#[singleton]
#[responds_to_http(method = Get, path = "/admin/users", server = "public")]
#[can(crate::action::Action::ManageUsers)]
#[traced]
pub struct GetAdminUsers {
    articles: Arc<ArticleRepository>,
    gatekeeper: Arc<Gatekeeper>,
    users: Arc<UserRepository>,
}

impl GetAdminUsers {
    #[constructor]
    pub fn create(
        articles: Arc<ArticleRepository>,
        gatekeeper: Arc<Gatekeeper>,
        users: Arc<UserRepository>,
    ) -> Self {
        Self {
            articles,
            gatekeeper,
            users,
        }
    }

    #[responder]
    pub async fn respond(&self, request: &Request) -> Response {
        let authentication = self.gatekeeper.authenticate(request).await;
        let context = self.gatekeeper.with_user(authentication);
        let articles = self.articles.all();
        let first = &articles[0];
        let read = context.can_read(first).await;
        let update = context.can_update(first).await;
        let delete = context.can_delete(first).await;
        let read_all = context.can_read_all(&articles).await;
        let update_all = context.can_update_all(&articles).await;
        let delete_all = context.can_delete_all(&articles).await;
        let names = self
            .users
            .all()
            .iter()
            .map(|user| user.name.clone())
            .collect::<Vec<String>>()
            .join(",");

        Response::text(
            200,
            format!(
                "users={names};read={read};update={update};delete={delete};read_all={read_all};update_all={update_all};delete_all={delete_all}"
            ),
        )
    }
}

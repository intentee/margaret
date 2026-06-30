use margaret_http::response::Response;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;
use margaret_security::authenticated_actor::AuthenticatedActor;

use crate::models::article::Article;
use crate::models::user::User;

#[singleton]
#[responds_to_http(method = Get, path = "/articles/{article}")]
pub struct GetArticle;

impl GetArticle {
    #[responder]
    pub async fn respond(
        &self,
        #[route_parameter(intent = CrudAction::Read)] article: Article,
        #[session_authenticated] viewer: AuthenticatedActor<User>,
    ) -> Response {
        let reader = match viewer {
            AuthenticatedActor::Anonymous => "a guest".to_string(),
            AuthenticatedActor::Session(user) => user.name.clone(),
        };

        Response::text(
            200,
            format!("{} reads \"{}\": {}", reader, article.title, article.body),
        )
    }
}

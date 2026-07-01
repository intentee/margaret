use margaret_http::response::Response;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;
use margaret_security::authenticated_actor::AuthenticatedActor;

use crate::models::article::Article;
use crate::models::user::User;

#[singleton]
#[responds_to_http(method = Get, path = "/articles/{article}", server = "public")]
pub struct GetArticle;

impl GetArticle {
    #[responder]
    pub async fn respond(
        &self,
        #[route_parameter(from = "article", intent = CrudAction::Read)] Article { title, body, .. }: Article,
        viewer: Option<AuthenticatedActor<User>>,
    ) -> Response {
        let reader = match viewer {
            None => "a guest".to_string(),
            Some(viewer) => viewer.actor.name.clone(),
        };

        Response::text(200, format!("{reader} reads \"{title}\": {body}"))
    }
}

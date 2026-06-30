use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::provides_authenticated_actor;
use margaret_macros::singleton;
use margaret_security::actor::Actor;
use margaret_security::authenticated_actor::AuthenticatedActor;
use margaret_security::authenticated_actor_store::AuthenticatedActorStore;
use margaret_security::session_authentication::SessionAuthentication;
use margaret_security::user_repository::UserRepository as UserRepositoryContract;

use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;

#[singleton]
#[provides_authenticated_actor]
pub struct SessionUserStore {
    repository: Arc<UserRepository>,
    session: SessionAuthentication,
}

impl SessionUserStore {
    #[constructor]
    pub fn create(repository: Arc<UserRepository>) -> Self {
        Self {
            repository,
            session: SessionAuthentication::new("margaret_session"),
        }
    }

    pub fn login(&self, request: &Request, response: Response, user: &User) -> Response {
        self.session
            .set_authenticated_actor(request, response, user.identifier())
    }

    pub fn logout(&self, request: &Request, response: Response) -> Response {
        self.session.forget(request, response)
    }
}

#[async_trait]
impl AuthenticatedActorStore for SessionUserStore {
    type Actor = User;

    async fn get_authenticated_actor(&self, request: &Request) -> AuthenticatedActor<User> {
        let user = match self.session.authenticated_actor_id(request) {
            Some(id) => self.repository.find_user_by_id(&id).await,
            None => None,
        };

        match user {
            Some(user) => AuthenticatedActor::Session(user),
            None => AuthenticatedActor::Anonymous,
        }
    }
}

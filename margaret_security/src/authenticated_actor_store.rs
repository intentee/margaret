use async_trait::async_trait;

use margaret_http::request::Request;

use crate::authenticated_actor::AuthenticatedActor;

#[async_trait]
pub trait AuthenticatedActorStore: Send + Sync {
    type Actor: crate::actor::Actor;

    async fn get_authenticated_actor(&self, request: &Request) -> AuthenticatedActor<Self::Actor>;
}

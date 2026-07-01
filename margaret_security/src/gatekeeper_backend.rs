use async_trait::async_trait;

use margaret_http::request::Request;

use crate::actor::Actor;
use crate::authenticated_actor::AuthenticatedActor;

#[async_trait]
pub trait GatekeeperBackend: Send + Sync {
    type Actor: Actor;

    async fn authenticate(&self, request: &Request) -> Option<AuthenticatedActor<Self::Actor>>;
}

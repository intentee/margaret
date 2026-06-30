use async_trait::async_trait;

use crate::authenticated_actor::AuthenticatedActor;

#[async_trait]
pub trait SiteActionGate: Send + Sync {
    type Actor: crate::actor::Actor;

    async fn can(&self, authenticated_actor: &AuthenticatedActor<Self::Actor>) -> bool;
}

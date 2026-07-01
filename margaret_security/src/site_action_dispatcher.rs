use async_trait::async_trait;

use crate::actor::Actor;
use crate::authenticated_actor::AuthenticatedActor;

#[async_trait]
pub trait SiteActionDispatcher<ActorType>: Send + Sync
where
    ActorType: Actor,
{
    type SiteAction;

    async fn can_site_action(
        &self,
        authenticated_actor: Option<&AuthenticatedActor<ActorType>>,
        action: Self::SiteAction,
    ) -> bool;
}

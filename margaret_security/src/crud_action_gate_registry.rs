use async_trait::async_trait;

use crate::actor::Actor;
use crate::authenticated_actor::AuthenticatedActor;
use crate::crud_action::CrudAction;

#[async_trait]
pub trait CrudActionGateRegistry<ActorType, Subject>
where
    ActorType: Actor,
{
    async fn can_crud(
        &self,
        authenticated_actor: Option<&AuthenticatedActor<ActorType>>,
        subject: &Subject,
        action: CrudAction,
    ) -> bool;
}

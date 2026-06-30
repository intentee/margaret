use async_trait::async_trait;

use crate::authenticated_actor::AuthenticatedActor;
use crate::crud_action::CrudAction;

#[async_trait]
pub trait CrudActionGate: Send + Sync {
    type Actor: crate::actor::Actor;
    type Subject;

    async fn can(
        &self,
        authenticated_actor: &AuthenticatedActor<Self::Actor>,
        subject: &Self::Subject,
        action: CrudAction,
    ) -> bool;
}

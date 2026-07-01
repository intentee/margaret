use std::sync::Arc;

use crate::authenticated_actor::AuthenticatedActor;
use crate::crud_action::CrudAction;
use crate::crud_action_gate_registry::CrudActionGateRegistry;
use crate::gatekeeper_backend::GatekeeperBackend;
use crate::site_action_dispatcher::SiteActionDispatcher;

pub struct GatekeeperUserContext<Backend>
where
    Backend: GatekeeperBackend,
{
    authenticated_actor: Option<AuthenticatedActor<Backend::Actor>>,
    backend: Arc<Backend>,
}

impl<Backend> GatekeeperUserContext<Backend>
where
    Backend: GatekeeperBackend,
{
    pub(crate) fn new(
        backend: Arc<Backend>,
        authenticated_actor: Option<AuthenticatedActor<Backend::Actor>>,
    ) -> Self {
        Self {
            authenticated_actor,
            backend,
        }
    }

    pub async fn can(
        &self,
        action: <Backend as SiteActionDispatcher<Backend::Actor>>::SiteAction,
    ) -> bool
    where
        Backend: SiteActionDispatcher<Backend::Actor>,
    {
        self.backend
            .can_site_action(self.authenticated_actor.as_ref(), action)
            .await
    }

    pub async fn can_crud<Subject>(&self, subject: &Subject, action: CrudAction) -> bool
    where
        Backend: CrudActionGateRegistry<Backend::Actor, Subject>,
    {
        self.backend
            .can_crud(self.authenticated_actor.as_ref(), subject, action)
            .await
    }

    pub async fn can_read<Subject>(&self, subject: &Subject) -> bool
    where
        Backend: CrudActionGateRegistry<Backend::Actor, Subject>,
    {
        self.can_crud(subject, CrudAction::Read).await
    }

    pub async fn can_update<Subject>(&self, subject: &Subject) -> bool
    where
        Backend: CrudActionGateRegistry<Backend::Actor, Subject>,
    {
        self.can_crud(subject, CrudAction::Update).await
    }

    pub async fn can_delete<Subject>(&self, subject: &Subject) -> bool
    where
        Backend: CrudActionGateRegistry<Backend::Actor, Subject>,
    {
        self.can_crud(subject, CrudAction::Delete).await
    }

    pub async fn can_crud_all<Subject>(&self, subjects: &[Subject], action: CrudAction) -> bool
    where
        Backend: CrudActionGateRegistry<Backend::Actor, Subject>,
    {
        for subject in subjects {
            if !self.can_crud(subject, action).await {
                return false;
            }
        }

        true
    }

    pub async fn can_read_all<Subject>(&self, subjects: &[Subject]) -> bool
    where
        Backend: CrudActionGateRegistry<Backend::Actor, Subject>,
    {
        self.can_crud_all(subjects, CrudAction::Read).await
    }

    pub async fn can_update_all<Subject>(&self, subjects: &[Subject]) -> bool
    where
        Backend: CrudActionGateRegistry<Backend::Actor, Subject>,
    {
        self.can_crud_all(subjects, CrudAction::Update).await
    }

    pub async fn can_delete_all<Subject>(&self, subjects: &[Subject]) -> bool
    where
        Backend: CrudActionGateRegistry<Backend::Actor, Subject>,
    {
        self.can_crud_all(subjects, CrudAction::Delete).await
    }
}

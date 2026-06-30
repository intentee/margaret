use std::sync::Arc;

use margaret_http::request::Request;

use crate::authenticated_actor::AuthenticatedActor;
use crate::crud_action::CrudAction;
use crate::crud_action_gate_registry::CrudActionGateRegistry;
use crate::gatekeeper_backend::GatekeeperBackend;
use crate::gatekeeper_user_context::GatekeeperUserContext;
use crate::site_action_dispatcher::SiteActionDispatcher;

pub struct Gatekeeper<Backend>
where
    Backend: GatekeeperBackend,
{
    backend: Arc<Backend>,
}

impl<Backend> Gatekeeper<Backend>
where
    Backend: GatekeeperBackend,
{
    pub fn new(backend: Arc<Backend>) -> Self {
        Self { backend }
    }

    pub async fn authenticate(&self, request: &Request) -> AuthenticatedActor<Backend::Actor> {
        self.backend.authenticate(request).await
    }

    pub async fn can_site_action(
        &self,
        authenticated_actor: &AuthenticatedActor<Backend::Actor>,
        action: <Backend as SiteActionDispatcher<Backend::Actor>>::SiteAction,
    ) -> bool
    where
        Backend: SiteActionDispatcher<Backend::Actor>,
    {
        self.backend
            .can_site_action(authenticated_actor, action)
            .await
    }

    pub async fn can_crud<Subject>(
        &self,
        authenticated_actor: &AuthenticatedActor<Backend::Actor>,
        subject: &Subject,
        action: CrudAction,
    ) -> bool
    where
        Backend: CrudActionGateRegistry<Backend::Actor, Subject>,
    {
        self.backend
            .can_crud(authenticated_actor, subject, action)
            .await
    }

    pub async fn with_request(&self, request: &Request) -> GatekeeperUserContext<Backend> {
        let authenticated_actor = self.authenticate(request).await;

        GatekeeperUserContext::new(self.backend.clone(), authenticated_actor)
    }

    pub fn with_user(
        &self,
        authenticated_actor: AuthenticatedActor<Backend::Actor>,
    ) -> GatekeeperUserContext<Backend> {
        GatekeeperUserContext::new(self.backend.clone(), authenticated_actor)
    }
}

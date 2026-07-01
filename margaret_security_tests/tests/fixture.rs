use async_trait::async_trait;

use margaret_http::method::Method;
use margaret_http::request::Request;
use margaret_security::actor::Actor;
use margaret_security::actor_role::ActorRole;
use margaret_security::authenticated_actor::AuthenticatedActor;
use margaret_security::crud_action::CrudAction;
use margaret_security::crud_action_gate_registry::CrudActionGateRegistry;
use margaret_security::gatekeeper_backend::GatekeeperBackend;
use margaret_security::site_action_dispatcher::SiteActionDispatcher;

#[derive(Clone, Copy)]
pub(crate) enum TestRole {
    Member,
    Admin,
}

impl ActorRole for TestRole {
    fn to_int(&self) -> u32 {
        match self {
            TestRole::Member => 0,
            TestRole::Admin => 1,
        }
    }
}

pub(crate) struct TestActor {
    identifier: String,
    role: TestRole,
}

impl TestActor {
    pub(crate) fn new(identifier: &str, role: TestRole) -> Self {
        Self {
            identifier: identifier.to_string(),
            role,
        }
    }
}

impl Actor for TestActor {
    type Role = TestRole;

    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn role(&self) -> TestRole {
        self.role
    }
}

pub(crate) enum TestSiteAction {
    Manage,
}

pub(crate) struct TestSubject {
    owner: String,
}

impl TestSubject {
    pub(crate) fn owned_by(owner: &str) -> Self {
        Self {
            owner: owner.to_string(),
        }
    }

    fn is_owned_by(&self, identifier: &str) -> bool {
        self.owner == identifier
    }
}

pub(crate) struct TestBackend;

#[async_trait]
impl GatekeeperBackend for TestBackend {
    type Actor = TestActor;

    async fn authenticate(&self, request: &Request) -> Option<AuthenticatedActor<TestActor>> {
        match request.path() {
            "/admin" => Some(AuthenticatedActor::new(TestActor::new(
                "admin",
                TestRole::Admin,
            ))),
            "/member" => Some(AuthenticatedActor::new(TestActor::new(
                "member",
                TestRole::Member,
            ))),
            _ => None,
        }
    }
}

#[async_trait]
impl SiteActionDispatcher<TestActor> for TestBackend {
    type SiteAction = TestSiteAction;

    async fn can_site_action(
        &self,
        authenticated_actor: Option<&AuthenticatedActor<TestActor>>,
        action: TestSiteAction,
    ) -> bool {
        match action {
            TestSiteAction::Manage => authenticated_actor
                .is_some_and(|actor| actor.actor.role().is_at_least(TestRole::Admin)),
        }
    }
}

#[async_trait]
impl CrudActionGateRegistry<TestActor, TestSubject> for TestBackend {
    async fn can_crud(
        &self,
        authenticated_actor: Option<&AuthenticatedActor<TestActor>>,
        subject: &TestSubject,
        action: CrudAction,
    ) -> bool {
        match authenticated_actor {
            None => matches!(action, CrudAction::Read),
            Some(actor) => {
                subject.is_owned_by(actor.actor.identifier())
                    || actor.actor.role().is_at_least(TestRole::Admin)
            }
        }
    }
}

pub(crate) fn request(path: &str) -> Request {
    Request::new(Method::Get, path.to_string())
}

use std::sync::Arc;

use margaret_security::crud_action::CrudAction;
use margaret_security::gatekeeper::Gatekeeper;

use crate::fixture::TestBackend;
use crate::fixture::TestSiteAction;
use crate::fixture::TestSubject;
use crate::fixture::request;

fn gatekeeper() -> Gatekeeper<TestBackend> {
    Gatekeeper::new(Arc::new(TestBackend))
}

#[tokio::test]
async fn authenticates_a_known_actor() {
    assert!(
        gatekeeper()
            .authenticate(&request("/admin"))
            .await
            .is_some()
    );
}

#[tokio::test]
async fn yields_no_actor_for_an_anonymous_request() {
    assert!(
        gatekeeper()
            .authenticate(&request("/guest"))
            .await
            .is_none()
    );
}

#[tokio::test]
async fn grants_a_site_action_to_an_authorized_actor() {
    let gatekeeper = gatekeeper();
    let actor = gatekeeper.authenticate(&request("/admin")).await;

    assert!(
        gatekeeper
            .can_site_action(actor.as_ref(), TestSiteAction::Manage)
            .await
    );
}

#[tokio::test]
async fn denies_a_site_action_to_an_anonymous_request() {
    assert!(
        !gatekeeper()
            .can_site_action(None, TestSiteAction::Manage)
            .await
    );
}

#[tokio::test]
async fn grants_crud_to_an_administrator_over_a_foreign_subject() {
    let gatekeeper = gatekeeper();
    let actor = gatekeeper.authenticate(&request("/admin")).await;

    assert!(
        gatekeeper
            .can_crud(
                actor.as_ref(),
                &TestSubject::owned_by("member"),
                CrudAction::Delete,
            )
            .await
    );
}

#[tokio::test]
async fn denies_a_write_to_an_anonymous_request() {
    assert!(
        !gatekeeper()
            .can_crud(None, &TestSubject::owned_by("admin"), CrudAction::Update)
            .await
    );
}

#[tokio::test]
async fn allows_an_anonymous_read() {
    assert!(
        gatekeeper()
            .can_crud(None, &TestSubject::owned_by("admin"), CrudAction::Read)
            .await
    );
}

#[tokio::test]
async fn builds_a_context_from_an_authenticated_actor() {
    let gatekeeper = gatekeeper();
    let actor = gatekeeper.authenticate(&request("/member")).await;
    let context = gatekeeper.with_user(actor);

    assert!(!context.can(TestSiteAction::Manage).await);
}

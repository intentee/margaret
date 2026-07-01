use std::sync::Arc;

use margaret_security::crud_action::CrudAction;
use margaret_security::gatekeeper::Gatekeeper;
use margaret_security::gatekeeper_user_context::GatekeeperUserContext;

use crate::fixture::TestBackend;
use crate::fixture::TestSiteAction;
use crate::fixture::TestSubject;
use crate::fixture::request;

async fn context_for(path: &str) -> GatekeeperUserContext<TestBackend> {
    Gatekeeper::new(Arc::new(TestBackend))
        .with_request(&request(path))
        .await
}

#[tokio::test]
async fn grants_an_owner_every_individual_crud_action() {
    let context = context_for("/member").await;
    let own = TestSubject::owned_by("member");

    assert!(context.can_crud(&own, CrudAction::Update).await);
    assert!(context.can_read(&own).await);
    assert!(context.can_update(&own).await);
    assert!(context.can_delete(&own).await);
}

#[tokio::test]
async fn grants_a_site_action_through_the_context() {
    let context = context_for("/admin").await;

    assert!(context.can(TestSiteAction::Manage).await);
}

#[tokio::test]
async fn approves_a_bulk_action_when_every_subject_passes() {
    let context = context_for("/admin").await;
    let subjects = vec![
        TestSubject::owned_by("member"),
        TestSubject::owned_by("other"),
    ];

    assert!(context.can_crud_all(&subjects, CrudAction::Delete).await);
    assert!(context.can_read_all(&subjects).await);
    assert!(context.can_update_all(&subjects).await);
    assert!(context.can_delete_all(&subjects).await);
}

#[tokio::test]
async fn rejects_a_bulk_action_when_one_subject_fails() {
    let context = context_for("/member").await;
    let subjects = vec![
        TestSubject::owned_by("member"),
        TestSubject::owned_by("other"),
    ];

    assert!(!context.can_update_all(&subjects).await);
}

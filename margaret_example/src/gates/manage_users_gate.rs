use async_trait::async_trait;

use margaret_macros::decides_site_action;
use margaret_macros::singleton;
use margaret_security::actor::Actor;
use margaret_security::actor_role::ActorRole;
use margaret_security::authenticated_actor::AuthenticatedActor;
use margaret_security::site_action_gate::SiteActionGate;

use crate::models::role::Role;
use crate::models::user::User;

#[singleton]
#[decides_site_action(crate::action::Action::ManageUsers)]
pub struct ManageUsersGate;

#[async_trait]
impl SiteActionGate for ManageUsersGate {
    type Actor = User;

    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>) -> bool {
        match authenticated_actor {
            AuthenticatedActor::Anonymous => false,
            AuthenticatedActor::Session(user) => user.role().is_at_least(Role::Admin),
        }
    }
}

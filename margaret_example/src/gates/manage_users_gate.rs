use margaret_macros::decides;
use margaret_macros::decides_site_action;
use margaret_macros::singleton;
use margaret_security::actor::Actor;
use margaret_security::actor_role::ActorRole;
use margaret_security::authenticated_actor::AuthenticatedActor;

use crate::models::role::Role;
use crate::models::user::User;

#[singleton]
#[decides_site_action(crate::action::Action::ManageUsers)]
pub struct ManageUsersGate;

impl ManageUsersGate {
    #[decides]
    pub async fn can(&self, AuthenticatedActor { actor: user }: &AuthenticatedActor<User>) -> bool {
        user.role().is_at_least(Role::Admin)
    }
}

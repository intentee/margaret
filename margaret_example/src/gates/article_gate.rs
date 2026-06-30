use async_trait::async_trait;

use margaret_macros::decides_crud_action;
use margaret_macros::singleton;
use margaret_security::actor::Actor;
use margaret_security::actor_role::ActorRole;
use margaret_security::authenticated_actor::AuthenticatedActor;
use margaret_security::crud_action::CrudAction;
use margaret_security::crud_action_gate::CrudActionGate;

use crate::models::article::Article;
use crate::models::role::Role;
use crate::models::user::User;

#[singleton]
#[decides_crud_action]
pub struct ArticleGate;

#[async_trait]
impl CrudActionGate for ArticleGate {
    type Actor = User;
    type Subject = Article;

    async fn can(
        &self,
        authenticated_actor: &AuthenticatedActor<User>,
        article: &Article,
        action: CrudAction,
    ) -> bool {
        match authenticated_actor {
            AuthenticatedActor::Anonymous => {
                matches!(action, CrudAction::Read) && article.published
            }
            AuthenticatedActor::Session(user) => match action {
                CrudAction::Read => {
                    article.published
                        || article.is_owned_by(user.identifier())
                        || user.role().is_at_least(Role::Moderator)
                }
                CrudAction::Update | CrudAction::Delete => {
                    article.is_owned_by(user.identifier()) || user.role().is_at_least(Role::Admin)
                }
            },
        }
    }
}

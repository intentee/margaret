use crate::actor_role::ActorRole;

pub trait Actor: Send + Sync + 'static {
    type Role: ActorRole;

    fn identifier(&self) -> &str;

    fn role(&self) -> Self::Role;
}

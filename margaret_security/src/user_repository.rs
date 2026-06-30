use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    type Actor: crate::actor::Actor;

    async fn find_user_by_id(&self, id: &str) -> Option<Self::Actor>;
}

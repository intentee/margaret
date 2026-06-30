use async_trait::async_trait;

use margaret_macros::singleton;
use margaret_security::user_repository::UserRepository as UserRepositoryContract;

use crate::models::role::Role;
use crate::models::user::User;

fn administrator() -> User {
    User {
        id: "1".to_string(),
        name: "Ada".to_string(),
        role: Role::Admin,
    }
}

fn moderator() -> User {
    User {
        id: "2".to_string(),
        name: "Mona".to_string(),
        role: Role::Moderator,
    }
}

fn member() -> User {
    User {
        id: "3".to_string(),
        name: "Milo".to_string(),
        role: Role::Member,
    }
}

#[singleton]
pub struct UserRepository;

impl UserRepository {
    pub fn all(&self) -> Vec<User> {
        vec![administrator(), moderator(), member()]
    }
}

#[async_trait]
impl UserRepositoryContract for UserRepository {
    type Actor = User;

    async fn find_user_by_id(&self, id: &str) -> Option<User> {
        match id {
            "1" => Some(administrator()),
            "2" => Some(moderator()),
            "3" => Some(member()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_security::user_repository::UserRepository as UserRepositoryContract;

    use super::UserRepository;

    #[tokio::test]
    async fn does_not_resolve_an_unknown_identifier() {
        assert!(UserRepository.find_user_by_id("404").await.is_none());
    }
}

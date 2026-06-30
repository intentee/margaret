use std::sync::Arc;

use margaret_macros::constructor;
use margaret_macros::singleton;
use margaret_security::user_repository::UserRepository as UserRepositoryContract;
use margaret_security::verify_password::verify_password;

use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;

const DEMO_PASSWORD_HASH: &str =
    "$argon2id$v=19$m=8,t=1,p=1$YWFhYWFhYWFhYWFhYWFhYQ$gyYhQZK0I0lX8SbZitOXV/bUkHHp+XQ9SL+ZsnELVaY";

#[singleton]
pub struct CredentialAuthenticator {
    repository: Arc<UserRepository>,
}

impl CredentialAuthenticator {
    #[constructor]
    pub fn create(repository: Arc<UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn authenticate(&self, username: &str, password: &str) -> Option<User> {
        let user_id = match username {
            "admin" => "1",
            "moderator" => "2",
            "member" => "3",
            _ => return None,
        };

        if !verify_password(password, DEMO_PASSWORD_HASH).expect("the demo password hash is valid")
        {
            return None;
        }

        self.repository.find_user_by_id(user_id).await
    }
}

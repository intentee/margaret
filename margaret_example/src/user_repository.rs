use margaret_macros::singleton;

use crate::models::user::User;

#[singleton]
pub struct UserRepository;

impl UserRepository {
    pub fn find(&self, id: &str) -> Option<User> {
        match id {
            "7" => Some(User {
                id: "7".to_string(),
                name: "Margaret".to_string(),
            }),
            _ => None,
        }
    }
}

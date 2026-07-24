use dashmap::DashMap;
use uuid::Uuid;

use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::models::user::User;

pub const MILO_SESSION: Uuid = Uuid::from_u128(900);

fn milo() -> User {
    User {
        id: Uuid::from_u128(3),
        name: "Milo".to_string(),
    }
}

#[singleton]
pub struct UserStore {
    sessions: DashMap<Uuid, User>,
}

impl UserStore {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        let sessions = DashMap::new();

        sessions.insert(MILO_SESSION, milo());

        Self { sessions }
    }

    #[must_use]
    pub fn find_user_by_session(&self, session: Uuid) -> Option<User> {
        self.sessions.get(&session).map(|user| user.value().clone())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::MILO_SESSION;
    use super::UserStore;

    #[test]
    fn finds_the_seeded_user_by_their_session() {
        assert_eq!(
            UserStore::create()
                .find_user_by_session(MILO_SESSION)
                .map(|user| user.name),
            Some("Milo".to_string())
        );
    }

    #[test]
    fn finds_no_user_for_an_unknown_session() {
        assert!(
            UserStore::create()
                .find_user_by_session(Uuid::from_u128(1))
                .is_none()
        );
    }
}

use std::sync::Arc;

use dashmap::DashMap;
use uuid::Uuid;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::models::user::User;
use crate::stores::user_session::UserSession;
use crate::system_clock::SystemClock;

pub use super::milo_session::MILO_SESSION;

const MILO: Uuid = Uuid::from_u128(3);

#[singleton]
pub struct UserStore {
    clock: Arc<SystemClock>,
    names: DashMap<Uuid, String>,
    sessions: DashMap<Uuid, UserSession>,
}

impl UserStore {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>) -> anyhow::Result<Self> {
        Ok({
            let names = DashMap::new();
            let sessions = DashMap::new();

            names.insert(MILO, "Milo".to_string());
            sessions.insert(
                MILO_SESSION,
                UserSession {
                    authenticated_at: clock.now(),
                    user_id: MILO,
                },
            );

            Self {
                clock,
                names,
                sessions,
            }
        })
    }

    #[must_use]
    pub fn find_user_by_session(&self, session: Uuid) -> Option<User> {
        self.sessions.get(&session).and_then(|session| {
            self.names.get(&session.user_id).map(|name| User {
                authenticated_at: session.authenticated_at,
                id: session.user_id,
                name: name.value().clone(),
            })
        })
    }

    #[must_use]
    pub fn find_user_name(&self, id: Uuid) -> Option<String> {
        self.names.get(&id).map(|name| name.value().clone())
    }

    #[must_use]
    pub fn start_session(&self, user_id: Uuid) -> Option<Uuid> {
        self.names.contains_key(&user_id).then(|| {
            let session = Uuid::new_v4();

            self.sessions.insert(
                session,
                UserSession {
                    authenticated_at: self.clock.now(),
                    user_id,
                },
            );

            session
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use uuid::Uuid;

    use super::MILO_SESSION;
    use super::UserStore;
    use crate::system_clock::SystemClock;

    #[test]
    fn finds_the_seeded_user_by_their_session() {
        assert_eq!(
            UserStore::create(Arc::new(SystemClock))
                .expect("the user store is constructed")
                .find_user_by_session(MILO_SESSION)
                .map(|user| user.name),
            Some("Milo".to_string())
        );
    }

    #[test]
    fn finds_no_user_for_an_unknown_session() {
        assert!(
            UserStore::create(Arc::new(SystemClock))
                .expect("the user store is constructed")
                .find_user_by_session(Uuid::from_u128(1))
                .is_none()
        );
    }
}

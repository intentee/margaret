use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

#[singleton]
#[infers_authenticated_user(user_model = crate::collision::reader::Reader)]
pub struct Session;

impl Session {
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self)
    }

    #[infer_from_request]
    pub async fn infer_reader(
        &self,
    ) -> anyhow::Result<AuthenticatedUserOutcome<crate::collision::reader::Reader>> {
        tokio::task::yield_now().await;

        Ok(AuthenticatedUserOutcome::Authenticated(
            crate::collision::reader::Reader {
                name: "milo".to_string(),
            },
        ))
    }
}

use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::creation::Creation;
use margaret::framework::active_record::detached::Detached;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

use crate::margaret::models::models_user_session_user_session::draft::Draft;
use crate::models::user_account::UserAccount;

#[model(table = "user_sessions")]
pub struct UserSession {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    pub authenticated_at: DateTime<Utc>,
    #[column]
    #[foreign_key(on_delete = OnDelete::Cascade)]
    #[index]
    pub user: Key<UserAccount>,
}

impl UserSession {
    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the session cannot be stored.
    pub async fn start(
        database: &Database,
        user: Uuid,
        authenticated_at: DateTime<Utc>,
    ) -> Result<Creation<Self>, ActiveRecordError> {
        Self::create(Draft {
            authenticated_at,
            user: Key::new(user),
        })
        .when(UserAccount::query().id.eq(user).exists::<Detached>())
        .run(database)
        .await
    }
}

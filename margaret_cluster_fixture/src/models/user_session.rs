use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

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
    pub user: UserAccount,
}

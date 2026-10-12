use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::models::user_account::UserAccount;

#[singleton]
pub struct UserAccountRepository {
    database: Arc<Database>,
}

impl UserAccountRepository {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the user account cannot be read.
    pub async fn find(&self, id: Uuid) -> Result<Lookup<UserAccount>, ActiveRecordError> {
        UserAccount::query()
            .id
            .eq(id)
            .find(self.database.as_ref())
            .await
    }
}

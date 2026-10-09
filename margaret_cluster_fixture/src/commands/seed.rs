use std::sync::Arc;

use margaret::framework::active_record::model::Model;
use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::database::database::Database;
use margaret::framework::database::isolation::Isolation;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use crate::alice::ALICE;
use crate::models::user_account::UserAccount;

#[singleton]
#[console_command(
    name = "seed",
    description = "Seeds the cluster database with its user"
)]
pub struct Seed {
    database: Arc<Database>,
}

impl Seed {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
    }

    /// # Errors
    ///
    /// Returns an error when the user cannot be stored.
    #[process]
    pub async fn run(&self) -> anyhow::Result<CommandOutcome> {
        let alice = UserAccount {
            id: ALICE,
            name: "alice".to_string(),
        };
        let mut connection = self.database.connection().await?;
        let transaction = connection.transaction(Isolation::ReadCommitted).await?;

        alice.insert().or_ignore(&transaction).await?;
        transaction.commit().await?;

        Ok(CommandOutcome::Succeeded)
    }
}

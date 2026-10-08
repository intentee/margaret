use std::sync::Arc;

use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::database::database::Database;
use margaret::framework::database::isolation::Isolation;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use crate::alice::ALICE;
use crate::alice_session::ALICE_SESSION;
use crate::models::user_account::UserAccount;
use crate::models::user_session::UserSession;
use crate::system_clock::SystemClock;

#[singleton]
#[console_command(
    name = "seed",
    description = "Seeds the cluster database with its user and session"
)]
pub struct Seed {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
}

impl Seed {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { clock, database })
    }

    /// # Errors
    ///
    /// Returns an error when the user or the session cannot be stored.
    #[process]
    pub async fn run(&self) -> anyhow::Result<CommandOutcome> {
        let alice = UserAccount {
            id: ALICE,
            name: "alice".to_string(),
        };
        let mut connection = self.database.connection().await?;
        let transaction = connection.transaction(Isolation::ReadCommitted).await?;

        alice.insert().or_ignore(&transaction).await?;
        UserSession {
            id: ALICE_SESSION,
            authenticated_at: self.clock.now(),
            user: Key::of(&alice),
        }
        .insert()
        .or_ignore(&transaction)
        .await?;
        transaction.commit().await?;

        Ok(CommandOutcome::Succeeded)
    }
}

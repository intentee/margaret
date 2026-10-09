use std::sync::Arc;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::database::database::Database;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use crate::margaret::models::models_note_note::draft::Draft;
use crate::models::note::Note;

#[singleton]
#[console_command(
    name = "write-note",
    description = "Writes a note to the cluster database"
)]
pub struct WriteNote {
    body: String,
    database: Arc<Database>,
}

impl WriteNote {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        database: Arc<Database>,
        #[console_argument(positional)] body: String,
    ) -> anyhow::Result<Self> {
        Ok(Self { body, database })
    }

    /// # Errors
    ///
    /// Returns an error when the note cannot be stored.
    #[process]
    pub async fn run(&self) -> anyhow::Result<CommandOutcome> {
        let note = Note::create(Draft {
            body: self.body.clone(),
            expires_at: None,
        })
        .run(self.database.as_ref())
        .await?;

        println!("{}", note.id);

        Ok(CommandOutcome::Succeeded)
    }
}

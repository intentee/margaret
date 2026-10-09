use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use crate::stores::note_store::NoteStore;

#[singleton]
#[console_command(
    name = "write-note",
    description = "Writes a note to the cluster database"
)]
pub struct WriteNote {
    body: String,
    notes: Arc<NoteStore>,
}

impl WriteNote {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        notes: Arc<NoteStore>,
        #[console_argument(positional)] body: String,
    ) -> anyhow::Result<Self> {
        Ok(Self { body, notes })
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the note cannot be stored.
    #[process]
    pub async fn run(&self) -> anyhow::Result<CommandOutcome> {
        let note = self.notes.insert(&self.body, None).await?;

        println!("{}", note.id);

        Ok(CommandOutcome::Succeeded)
    }
}

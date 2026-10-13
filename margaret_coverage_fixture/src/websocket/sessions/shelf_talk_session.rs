use tokio::sync::Mutex;

use margaret::framework::macros::build_for_session;
use margaret::framework::macros::websocket_session;

use crate::auth::authenticated_users::librarian::Librarian;
use crate::route_parameters::book_number::BookNumber;
use crate::route_parameters::shelf_label::ShelfLabel;

#[websocket_session(path = "/talk/{shelf}/{number}", server = "public")]
pub struct ShelfTalkSession {
    librarian: Librarian,
    number: u32,
    shelf: String,
    whispers: Mutex<usize>,
}

impl ShelfTalkSession {
    /// # Errors
    ///
    /// Never returns an error; every admitted librarian may talk at a shelf.
    #[build_for_session]
    pub fn build_for_session(
        #[route_parameter(from = "shelf")] ShelfLabel(shelf): ShelfLabel,
        #[route_parameter(from = "number")] BookNumber(number): BookNumber,
        #[authenticated_user] librarian: Librarian,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            librarian,
            number,
            shelf,
            whispers: Mutex::new(0),
        })
    }

    pub async fn hear_whisper(&self) {
        *self.whispers.lock().await += 1;
    }

    #[must_use]
    pub fn librarian_badge(&self) -> &str {
        &self.librarian.badge
    }

    #[must_use]
    pub fn shelf_location(&self) -> String {
        format!("{} #{}", self.shelf, self.number)
    }

    pub async fn whispers(&self) -> usize {
        *self.whispers.lock().await
    }
}

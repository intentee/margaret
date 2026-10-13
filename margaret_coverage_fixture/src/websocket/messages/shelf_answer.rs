use serde::Serialize;

use margaret::framework::macros::websocket_message;

#[websocket_message(response, method = "shelf_answer")]
#[derive(Serialize)]
pub struct ShelfAnswer {
    pub answer: String,
}

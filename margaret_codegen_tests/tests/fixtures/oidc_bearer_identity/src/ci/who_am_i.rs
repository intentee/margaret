use serde::Deserialize;
use validator::Validate;

use margaret::framework::macros::websocket_message;

#[websocket_message(request, method = "who_am_i", response = single)]
#[derive(Deserialize, Validate)]
pub struct WhoAmI {}

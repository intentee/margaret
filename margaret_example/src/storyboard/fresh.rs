use margaret_macros::websocket_state;

#[derive(Default)]
#[websocket_state(server = "public", path = "/storyboard")]
pub struct Fresh;

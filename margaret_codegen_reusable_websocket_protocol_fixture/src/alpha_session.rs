pub mod on_conversation_message;

use margaret::framework::macros::build_for_session;
use margaret::framework::macros::websocket_session;

use crate::margaret::asset_bag::asset;

#[websocket_session(path = "/alpha", server = "public")]
pub struct AlphaSession;

impl AlphaSession {
    #[build_for_session]
    pub fn new() -> anyhow::Result<Self> {
        Ok({
            let _ = asset!("resources/ts/app.ts");

            Self
        })
    }
}

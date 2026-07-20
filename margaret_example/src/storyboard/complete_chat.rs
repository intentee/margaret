use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::singleton;
use margaret_macros::websocket_transition;

use crate::storyboard::ended::Ended;
use crate::storyboard::generation_complete::GenerationComplete;
use crate::storyboard::thinking::Thinking;

#[singleton]
#[websocket_transition(
    from = crate::storyboard::thinking::Thinking,
    on = crate::storyboard::generation_complete::GenerationComplete
)]
pub struct CompleteChat;

impl CompleteChat {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    #[process]
    pub async fn on(&self, _state: Thinking, _event: GenerationComplete) -> Ended {
        Ended
    }
}

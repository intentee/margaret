use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::singleton;
use margaret_macros::websocket_transition;
use margaret_websocket::activity_spawner::ActivitySpawner;
use margaret_websocket::emit::Emit;
use margaret_websocket::envelope::Envelope;

use crate::margaret::websocket::storyboard_fresh_fresh::BeginChatEmit;
use crate::margaret::websocket::storyboard_fresh_fresh::Internal;
use crate::storyboard::accepted::Accepted;
use crate::storyboard::fresh::Fresh;
use crate::storyboard::generation_complete::GenerationComplete;
use crate::storyboard::speak::Speak;
use crate::storyboard::thinking::Thinking;

#[singleton]
#[websocket_transition(
    from = crate::storyboard::fresh::Fresh,
    on = crate::storyboard::speak::Speak,
    emits(crate::storyboard::accepted::Accepted)
)]
pub struct BeginChat;

impl BeginChat {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    #[process]
    pub async fn on(
        &self,
        _state: Fresh,
        message: Envelope<Speak>,
        emit: &Emit<BeginChatEmit>,
        spawner: &ActivitySpawner<Internal>,
    ) -> Thinking {
        emit.push(Accepted {
            echoed: message.message.text,
        })
        .await;

        let internal_sender = spawner.internal_sender();

        spawner.spawn(async move {
            let _ = internal_sender.send(Internal::from(GenerationComplete)).await;
        });

        Thinking
    }
}

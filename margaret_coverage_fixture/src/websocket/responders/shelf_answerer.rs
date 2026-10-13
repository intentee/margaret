use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret::framework::macros::singleton;
use margaret::framework::websocket::request_envelope::RequestEnvelope;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::web_socket::WebSocket;

use crate::websocket::messages::ask_shelf::AskShelf;
use crate::websocket::messages::shelf_answer::ShelfAnswer;
use crate::websocket::sessions::shelf_talk_session::ShelfTalkSession;

#[singleton]
pub struct ShelfAnswerer;

#[async_trait]
impl RespondsToWebSocketMessage for ShelfAnswerer {
    type Message = AskShelf;
    type Session = ShelfTalkSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        session: Arc<ShelfTalkSession>,
        message: RequestEnvelope<AskShelf>,
        socket: WebSocket,
    ) -> anyhow::Result<()> {
        let answer = ShelfAnswer {
            answer: format!(
                "{} answers '{}' at {} after {} whispers",
                session.librarian_badge(),
                message.message().question,
                session.shelf_location(),
                session.whispers().await,
            ),
        };

        socket
            .send(message.response(answer))
            .await
            .map_err(anyhow::Error::from)
    }
}

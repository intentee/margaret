use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret::framework::macros::singleton;
use margaret::framework::websocket::request_envelope::RequestEnvelope;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::web_socket::WebSocket;

use crate::room::Room;
use crate::room_chat::RoomChat;
use crate::room_reply::RoomReply;

#[singleton]
pub struct RoomChatter;

#[async_trait]
impl RespondsToWebSocketMessage for RoomChatter {
    type Message = RoomChat;
    type Session = Room;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        _session: Arc<Room>,
        message: RequestEnvelope<RoomChat>,
        socket: WebSocket,
    ) -> anyhow::Result<()> {
        socket
            .send(message.response(RoomReply {
                text: message.message().text.clone(),
            }))
            .await?;

        Ok(())
    }
}

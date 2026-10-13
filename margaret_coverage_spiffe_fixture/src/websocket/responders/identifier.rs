use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret::framework::macros::singleton;
use margaret::framework::websocket::request_envelope::RequestEnvelope;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::web_socket::WebSocket;

use crate::websocket::messages::identify::Identify;
use crate::websocket::messages::identity::Identity;
use crate::websocket::sessions::mesh_talk_session::MeshTalkSession;

#[singleton]
pub struct Identifier;

#[async_trait]
impl RespondsToWebSocketMessage for Identifier {
    type Message = Identify;
    type Session = MeshTalkSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        session: Arc<MeshTalkSession>,
        message: RequestEnvelope<Identify>,
        socket: WebSocket,
    ) -> anyhow::Result<()> {
        socket
            .send(message.response(Identity {
                spiffe_id: session.peer().to_string(),
            }))
            .await
            .map_err(anyhow::Error::from)
    }
}

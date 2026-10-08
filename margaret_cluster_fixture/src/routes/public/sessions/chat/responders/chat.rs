use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret::framework::websocket::web_socket::WebSocket;

use crate::margaret::models::models_message_message::draft::Draft;
use crate::models::message::Message;
use crate::routes::public::sessions::chat::chat_session::ChatSession;
use crate::routes::public::sessions::chat::messages::post_message::PostMessage;
use crate::routes::public::sessions::chat::messages::posted_message::PostedMessage;
use crate::system_clock::SystemClock;

#[singleton]
pub struct Chat {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
}

impl Chat {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { clock, database })
    }
}

#[async_trait]
impl RespondsToWebSocketMessage for Chat {
    type Message = PostMessage;
    type Session = ChatSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        _session: Arc<ChatSession>,
        message: StreamingRequestEnvelope<PostMessage>,
        socket: WebSocket,
    ) -> anyhow::Result<()> {
        let posted = Message::create(Draft {
            body: message.message().body.clone(),
            posted_at: self.clock.now(),
        })
        .run(self.database.as_ref())
        .await?;

        socket
            .send(message.fin(PostedMessage { id: posted.id }))
            .await?;

        Ok(())
    }
}

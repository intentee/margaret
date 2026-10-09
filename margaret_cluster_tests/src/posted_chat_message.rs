use futures_util::SinkExt as _;
use futures_util::StreamExt as _;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;

/// # Panics
///
/// Panics when the chat session does not answer the message.
pub async fn posted_chat_message(
    socket: &mut WebSocketStream<TlsStream<TcpStream>>,
    body: &str,
) -> Message {
    socket
        .send(Message::text(
            json!({ "id": 1, "method": "post_message", "params": { "body": body } }).to_string(),
        ))
        .await
        .expect("the chat message is sent");

    socket
        .next()
        .await
        .expect("the chat session answers")
        .expect("the chat answer reads cleanly")
}

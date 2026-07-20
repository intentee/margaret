use futures_util::SinkExt;
use futures_util::StreamExt;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::client_async;
use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_tests::running_echo_server::RunningEchoServer;

async fn read_json(websocket: &mut WebSocketStream<TcpStream>) -> Value {
    let message = websocket
        .next()
        .await
        .expect("a frame arrives")
        .expect("the frame is not a transport error");
    let text = message.to_text().expect("the frame is a text frame");

    serde_json::from_str(text).expect("the frame is valid json")
}

#[tokio::test]
async fn drives_a_websocket_conversation() {
    let server = RunningEchoServer::start().await;
    let address = server.address();

    let stream = TcpStream::connect(address)
        .await
        .expect("the client connects to the server");
    let (mut websocket, _response) = client_async(format!("ws://{address}/ws"), stream)
        .await
        .expect("the websocket handshake completes");

    websocket
        .send(Message::text(
            "{\"jsonrpc\":\"2.0\",\"method\":\"echo\",\"id\":1}",
        ))
        .await
        .expect("the request is sent");

    let result = read_json(&mut websocket).await;

    assert_eq!(result["id"], 1);
    assert_eq!(result["result"], "echo");

    let tick = read_json(&mut websocket).await;

    assert_eq!(tick["method"], "tick");

    websocket
        .send(Message::text("{\"jsonrpc\":\"2.0\",\"method\":\"stop\"}"))
        .await
        .expect("the stop notification is sent");

    let closed = loop {
        match websocket.next().await {
            Some(Ok(message)) if message.is_close() => break true,
            Some(Ok(_)) => continue,
            Some(Err(_)) | None => break true,
        }
    };

    assert!(closed, "the server closes the connection on the terminal state");

    server.stop().await;
}

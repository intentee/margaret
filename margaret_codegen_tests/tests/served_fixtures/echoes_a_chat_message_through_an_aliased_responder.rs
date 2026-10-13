use futures_util::SinkExt;
use futures_util::StreamExt;
use serde_json::Value;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::client_async;
use tokio_tungstenite::tungstenite::Message;

use margaret_codegen_fallible_roles_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_codegen_tests::served_fallible_roles::ServedFallibleRoles;
use margaret_codegen_tests::visit_fallible_roles::visit_fallible_roles;

#[tokio::test]
async fn echoes_a_chat_message_through_an_aliased_responder() {
    let outcome = visit_fallible_roles(
        async |ServedFallibleRoles { address, .. }: &ServedFallibleRoles| {
            let (mut chat, _) = client_async(
                format!("ws://{address}/chat"),
                TcpStream::connect(address)
                    .await
                    .expect("the chat accepts the connection"),
            )
            .await
            .expect("the chat session opens");

            chat.send(Message::text(
                r#"{"id":3,"method":"chat_say","params":{"text":"hello"}}"#,
            ))
            .await
            .expect("the chat message is sent");

            let echo = chat
                .next()
                .await
                .expect("the chat answers")
                .expect("the answer arrives intact");

            assert_eq!(
                serde_json::from_str::<Value>(echo.to_text().expect("the answer is text"))
                    .expect("the answer is json"),
                json!({
                    "done": true,
                    "id": 3,
                    "method": "chat_echo",
                    "result": { "text": "hello" },
                })
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

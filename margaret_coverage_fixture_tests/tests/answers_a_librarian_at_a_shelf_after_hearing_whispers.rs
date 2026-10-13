use futures_util::SinkExt;
use futures_util::StreamExt;
use serde_json::Value;
use serde_json::json;
use tempfile::tempdir;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;

use margaret_coverage_fixture::auth::headers::librarian_badge::LIBRARIAN_BADGE;
use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture_tests::listed_librarian::LISTED_LIBRARIAN;
use margaret_coverage_fixture_tests::open_websocket::open_websocket;
use margaret_coverage_fixture_tests::served_library::ServedLibrary;
use margaret_coverage_fixture_tests::visit_library::visit_library;
use margaret_coverage_fixture_tests::written_library_records::written_library_records;

#[tokio::test]
async fn answers_a_librarian_at_a_shelf_after_hearing_whispers() {
    let directory = tempdir().expect("the records directory is created");

    let outcome = visit_library(
        written_library_records(directory.path()),
        async |ServedLibrary { address, .. }: &ServedLibrary| {
            let mut request = format!("ws://{address}/talk/fiction/42")
                .into_client_request()
                .expect("the talk address is a websocket request");

            request
                .headers_mut()
                .insert(LIBRARIAN_BADGE, HeaderValue::from_static(LISTED_LIBRARIAN));

            let (mut talk, _) = open_websocket(*address, request)
                .await
                .expect("the listed librarian talks at the shelf");

            talk.send(Message::text(r#"{"method":"whisper","params":{"text":"psst"}}"#))
                .await
                .expect("the whisper is sent");
            talk.send(Message::text(
                r#"{"id":1,"method":"ask_shelf","params":{"question":"where is Dune"}}"#,
            ))
            .await
            .expect("the question is sent");

            let answer = talk
                .next()
                .await
                .expect("the shelf answers")
                .expect("the answer arrives intact");

            assert_eq!(
                serde_json::from_str::<Value>(answer.to_text().expect("the answer is text"))
                    .expect("the answer is json"),
                json!({
                    "done": true,
                    "id": 1,
                    "method": "shelf_answer",
                    "result": {
                        "answer": format!("{LISTED_LIBRARIAN} answers 'where is Dune' at fiction #42 after 1 whispers"),
                    },
                })
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

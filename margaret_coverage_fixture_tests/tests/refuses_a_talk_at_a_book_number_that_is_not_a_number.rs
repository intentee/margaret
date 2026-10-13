use tempfile::tempdir;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::http::StatusCode;

use margaret_coverage_fixture::auth::headers::librarian_badge::LIBRARIAN_BADGE;
use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture_tests::listed_librarian::LISTED_LIBRARIAN;
use margaret_coverage_fixture_tests::open_websocket::open_websocket;
use margaret_coverage_fixture_tests::served_library::ServedLibrary;
use margaret_coverage_fixture_tests::visit_library::visit_library;
use margaret_coverage_fixture_tests::written_library_records::written_library_records;

#[tokio::test]
async fn refuses_a_talk_at_a_book_number_that_is_not_a_number() {
    let directory = tempdir().expect("the records directory is created");

    let outcome = visit_library(
        written_library_records(directory.path()),
        async |ServedLibrary { address, .. }: &ServedLibrary| {
            let mut request = format!("ws://{address}/talk/fiction/first")
                .into_client_request()
                .expect("the talk address is a websocket request");

            request
                .headers_mut()
                .insert(LIBRARIAN_BADGE, HeaderValue::from_static(LISTED_LIBRARIAN));

            assert!(matches!(
                open_websocket(*address, request).await,
                Err(Error::Http(refusal)) if refusal.status() == StatusCode::NOT_FOUND
            ));
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

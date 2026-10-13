use tempfile::tempdir;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::http::StatusCode;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture_tests::open_websocket::open_websocket;
use margaret_coverage_fixture_tests::served_library::ServedLibrary;
use margaret_coverage_fixture_tests::visit_library::visit_library;
use margaret_coverage_fixture_tests::written_library_records::written_library_records;

#[tokio::test]
async fn challenges_an_anonymous_talk_at_a_shelf() {
    let directory = tempdir().expect("the records directory is created");

    let outcome = visit_library(
        written_library_records(directory.path()),
        async |ServedLibrary { address, .. }: &ServedLibrary| {
            assert!(matches!(
                open_websocket(*address, format!("ws://{address}/talk/fiction/42")).await,
                Err(Error::Http(refusal)) if refusal.status() == StatusCode::UNAUTHORIZED
            ));
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

use reqwest::StatusCode;
use tempfile::tempdir;

use margaret_coverage_fixture::auth::headers::patron_card::PATRON_CARD;
use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture_tests::answer::Answer;
use margaret_coverage_fixture_tests::served_library::ServedLibrary;
use margaret_coverage_fixture_tests::visit_library::visit_library;
use margaret_coverage_fixture_tests::written_library_records::written_library_records;

#[tokio::test]
async fn refuses_a_malformed_patron_card_at_the_reading_room() {
    let directory = tempdir().expect("the records directory is created");

    let outcome = visit_library(
        written_library_records(directory.path()),
        async |ServedLibrary { address, client }: &ServedLibrary| {
            assert_eq!(
                Answer::of(
                    client
                        .get(format!("http://{address}/reading-room"))
                        .header(PATRON_CARD, "seven")
                )
                .await,
                Answer {
                    body: "the patron card is not a card number: invalid digit found in string"
                        .to_string(),
                    status: StatusCode::BAD_REQUEST,
                }
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

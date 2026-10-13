use reqwest::StatusCode;
use tempfile::tempdir;

use margaret_coverage_fixture::constants::PATRON_CARD;
use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture_tests::answer::Answer;
use margaret_coverage_fixture_tests::served_library::ServedLibrary;
use margaret_coverage_fixture_tests::visit_library::visit_library;
use margaret_coverage_fixture_tests::written_library_records::written_library_records;

#[tokio::test]
async fn lists_the_loans_of_a_patron_card() {
    let directory = tempdir().expect("the records directory is created");

    let outcome = visit_library(
        written_library_records(directory.path()),
        async |ServedLibrary { address, client }: &ServedLibrary| {
            assert_eq!(
                Answer::of(
                    client
                        .get(format!("http://{address}/loans"))
                        .header(PATRON_CARD, "7")
                )
                .await,
                Answer {
                    body: "loans of card 7".to_string(),
                    status: StatusCode::OK,
                }
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

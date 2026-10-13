use reqwest::StatusCode;
use tempfile::tempdir;

use margaret_coverage_fixture::constants::LIBRARIAN_BADGE;
use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture_tests::answer::Answer;
use margaret_coverage_fixture_tests::constants::LISTED_LIBRARIAN;
use margaret_coverage_fixture_tests::library_records::LibraryRecords;
use margaret_coverage_fixture_tests::served_library::ServedLibrary;
use margaret_coverage_fixture_tests::visit_library::visit_library;
use margaret_coverage_fixture_tests::written_library_records::written_library_records;

#[tokio::test]
async fn fails_the_staff_room_without_a_roster() {
    let directory = tempdir().expect("the records directory is created");

    let outcome = visit_library(
        LibraryRecords {
            roster: directory.path().join("missing-roster"),
            ..written_library_records(directory.path())
        },
        async |ServedLibrary { address, client }: &ServedLibrary| {
            assert_eq!(
                Answer::of(
                    client
                        .get(format!("http://{address}/staff-room"))
                        .header(LIBRARIAN_BADGE, LISTED_LIBRARIAN)
                )
                .await,
                Answer {
                    body: "Internal Server Error".to_string(),
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                }
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

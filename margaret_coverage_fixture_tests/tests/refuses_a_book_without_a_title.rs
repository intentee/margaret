use reqwest::StatusCode;
use tempfile::tempdir;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture_tests::answer::Answer;
use margaret_coverage_fixture_tests::served_library::ServedLibrary;
use margaret_coverage_fixture_tests::visit_library::visit_library;
use margaret_coverage_fixture_tests::written_library_records::written_library_records;

#[tokio::test]
async fn refuses_a_book_without_a_title() {
    let directory = tempdir().expect("the records directory is created");

    let outcome = visit_library(
        written_library_records(directory.path()),
        async |ServedLibrary { address, client }: &ServedLibrary| {
            assert_eq!(
                Answer::of(
                    client
                        .post(format!("http://{address}/books"))
                        .form(&[("title", "")])
                )
                .await
                .status,
                StatusCode::UNPROCESSABLE_ENTITY
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

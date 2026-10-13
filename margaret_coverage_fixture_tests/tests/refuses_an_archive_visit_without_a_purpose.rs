use reqwest::StatusCode;
use tempfile::tempdir;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture_tests::answer::Answer;
use margaret_coverage_fixture_tests::served_library::ServedLibrary;
use margaret_coverage_fixture_tests::visit_library::visit_library;
use margaret_coverage_fixture_tests::written_library_records::written_library_records;

#[tokio::test]
async fn refuses_an_archive_visit_without_a_purpose() {
    let directory = tempdir().expect("the records directory is created");

    let outcome = visit_library(
        written_library_records(directory.path()),
        async |ServedLibrary { address, client }: &ServedLibrary| {
            assert_eq!(
                Answer::of(client.get(format!("http://{address}/archive"))).await,
                Answer {
                    body: "the request input could not be interpreted: missing field `purpose`"
                        .to_string(),
                    status: StatusCode::BAD_REQUEST,
                }
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

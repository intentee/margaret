use reqwest::StatusCode;
use tempfile::tempdir;

use margaret_coverage_fixture::constants::COURIER_NAME;
use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture_tests::answer::Answer;
use margaret_coverage_fixture_tests::constants::LISTED_COURIER;
use margaret_coverage_fixture_tests::served_library::ServedLibrary;
use margaret_coverage_fixture_tests::visit_library::visit_library;
use margaret_coverage_fixture_tests::written_library_records::written_library_records;

#[tokio::test]
async fn assigns_the_loading_dock_to_a_listed_courier() {
    let directory = tempdir().expect("the records directory is created");

    let outcome = visit_library(
        written_library_records(directory.path()),
        async |ServedLibrary { address, client }: &ServedLibrary| {
            assert_eq!(
                Answer::of(
                    client
                        .get(format!("http://{address}/loading-dock"))
                        .header(COURIER_NAME, LISTED_COURIER)
                )
                .await,
                Answer {
                    body: format!("dock assigned to {LISTED_COURIER}"),
                    status: StatusCode::OK,
                }
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

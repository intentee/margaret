use tempfile::tempdir;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture_tests::answer::Answer;
use margaret_coverage_fixture_tests::served_library::ServedLibrary;
use margaret_coverage_fixture_tests::visit_library::visit_library;
use margaret_coverage_fixture_tests::written_library_records::written_library_records;
use margaret_local_server_tests::poll_until::poll_until;

#[tokio::test]
async fn audits_the_shelves_when_the_library_opens() {
    let directory = tempdir().expect("the records directory is created");

    let outcome = visit_library(
        written_library_records(directory.path()),
        async |ServedLibrary { address, client }: &ServedLibrary| {
            poll_until(async || {
                Answer::of(client.get(format!("http://{address}/audits")))
                    .await
                    .body
                    == "1"
            })
            .await;
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}

use tempfile::tempdir;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture::margaret::run::run;
use margaret_coverage_fixture_tests::library_serve_arguments::library_serve_arguments;
use margaret_coverage_fixture_tests::written_library_records::written_library_records;
use margaret_local_server_tests::reserved_address::reserved_address;

#[tokio::test]
async fn refuses_to_serve_hours_that_close_before_opening() {
    let directory = tempdir().expect("the records directory is created");

    assert_eq!(
        run(library_serve_arguments(
            reserved_address(),
            written_library_records(directory.path()),
            "18",
            "9",
        ))
        .await,
        CommandOutcome::Failed
    );
}

use std::ffi::OsStr;

use tempfile::tempdir;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture::margaret::run::run;

#[tokio::test]
async fn fails_to_count_books_without_an_inventory() {
    let directory = tempdir().expect("the inventory directory is created");
    let count = directory.path().join("count");

    assert_eq!(
        run([
            OsStr::new("margaret_coverage_fixture"),
            OsStr::new("count-books"),
            OsStr::new("--inventory"),
            directory.path().join("missing-inventory").as_os_str(),
            OsStr::new("--to"),
            count.as_os_str(),
        ])
        .await,
        CommandOutcome::Failed
    );
    assert!(!count.exists());
}

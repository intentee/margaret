use std::ffi::OsStr;
use std::fs;

use tempfile::tempdir;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture::margaret::run::run;

#[tokio::test]
async fn counts_the_books_of_an_inventory() {
    let directory = tempdir().expect("the inventory directory is created");
    let inventory = directory.path().join("inventory");
    let count = directory.path().join("count");

    fs::write(&inventory, "Dune\nEmma\nUlysses\n").expect("the inventory is written");

    assert_eq!(
        run([
            OsStr::new("margaret_coverage_fixture"),
            OsStr::new("count-books"),
            OsStr::new("--inventory"),
            inventory.as_os_str(),
            OsStr::new("--to"),
            count.as_os_str(),
        ])
        .await,
        CommandOutcome::Succeeded
    );
    assert_eq!(
        fs::read_to_string(&count).expect("the count is written"),
        "3"
    );
}

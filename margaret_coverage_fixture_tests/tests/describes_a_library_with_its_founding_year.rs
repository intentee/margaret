use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;

use tempfile::tempdir;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture_tests::run_to_completion::run_to_completion;
use margaret_process_tests::child_variable::ChildVariable;
use margaret_process_tests::in_child_process::in_child_process;

#[test]
fn describes_a_library_with_its_founding_year() {
    in_child_process(
        "describes_a_library_with_its_founding_year::describes_a_library_with_its_founding_year",
        &[
            ChildVariable::Set {
                name: "MARGARET_COVERAGE_FIXTURE_LIBRARY_NAME",
                value: OsString::from("Alexandria"),
            },
            ChildVariable::Set {
                name: "MARGARET_COVERAGE_FIXTURE_LIBRARY_FOUNDED",
                value: OsString::from("1863"),
            },
        ],
        || {
            let directory = tempdir().expect("the description directory is created");
            let description = directory.path().join("description");

            assert_eq!(
                run_to_completion([
                    OsStr::new("margaret_coverage_fixture"),
                    OsStr::new("describe-library"),
                    OsStr::new("--to"),
                    description.as_os_str(),
                ]),
                CommandOutcome::Succeeded
            );
            assert_eq!(
                fs::read_to_string(&description).expect("the description is written"),
                "Alexandria (founded 1863)"
            );
        },
    );
}

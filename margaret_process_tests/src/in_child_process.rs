use std::env;
use std::fs;
use std::process::Command;

use tempfile::tempdir;

use crate::child_variable::ChildVariable;

const CHILD_COMPLETION_VARIABLE: &str = "MARGARET_CHILD_TEST_COMPLETION";

/// # Panics
///
/// Panics when the child process cannot run the test, the test fails in it, or the test never
/// reaches its end there.
pub fn in_child_process(test_name: &str, variables: &[ChildVariable], body: impl FnOnce()) {
    if let Some(completion) = env::var_os(CHILD_COMPLETION_VARIABLE) {
        body();
        fs::write(completion, test_name).expect("the child records its completion");
    } else {
        let directory = tempdir().expect("the completion directory is created");
        let completion = directory.path().join("completed");
        let mut command = Command::new(env::current_exe().expect("the test binary is known"));

        command
            .arg(test_name)
            .arg("--exact")
            .env(CHILD_COMPLETION_VARIABLE, &completion);

        for variable in variables {
            variable.apply(&mut command);
        }

        assert!(
            command
                .status()
                .expect("the child test process runs")
                .success()
        );
        assert_eq!(
            fs::read_to_string(&completion).expect("the child completes the test"),
            test_name
        );
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::ffi::OsString;
    use std::thread;

    use super::in_child_process;
    use crate::child_variable::ChildVariable;

    #[test]
    fn runs_the_body_with_its_variables() {
        in_child_process(
            "in_child_process::tests::runs_the_body_with_its_variables",
            &[
                ChildVariable::Set {
                    name: "MARGARET_CHILD_SET",
                    value: OsString::from("set"),
                },
                ChildVariable::Removed("PATH"),
            ],
            || {
                assert_eq!(env::var("MARGARET_CHILD_SET").as_deref(), Ok("set"));
                assert_eq!(env::var_os("PATH"), None);
            },
        );
    }

    #[test]
    #[should_panic(expected = "the child completes the test")]
    fn refuses_a_child_that_never_reaches_the_test() {
        in_child_process(
            "in_child_process::tests::no_such_test",
            &[],
            thread::yield_now,
        );
    }
}
